//! ENC28J60 + smoltcp demo
//!
//! Demonstrates how to use an ENC28J60 with smoltcp by running a simple demo that
//! toggles and returns the current LED state.
//!
//! You can test this program with the following:
//!
//! - `ping 192.168.10.33`. The device will respond to every request (response time should be ~10ms).
//! - `curl 192.168.10.33`. The device will respond with a HTTP response with the current
//! LED state in the body.
//! - Visiting `https://192.168.10.33/`. Every refresh will toggle the LED and the page will
//! reflect the current state.
//!
//! Note that you might need to change the ip address for your local network in order for things to work.
//!

#![no_std]
#![no_main]
#![allow(deprecated)]

use core::fmt::Write;
use enc28j60::{smoltcp_phy::Phy, Enc28j60, Unconnected};
use esp8266_hal::ehal::digital::v1_compat::{OldInputPin, OldOutputPin};
use esp8266_hal::ehal::digital::v2::StatefulOutputPin;
use esp8266_hal::prelude::*;
use esp8266_hal::spi::SpiClock;
use esp8266_hal::target::Peripherals;
use panic_halt as _;
use smoltcp::iface::{EthernetInterfaceBuilder, NeighborCache};
use smoltcp::socket::{SocketSet, TcpSocket, TcpSocketBuffer};
use smoltcp::time::Instant;
use smoltcp::wire;
use smoltcp::wire::{EthernetAddress, IpAddress, IpCidr, Ipv4Address};

/* Configuration, adjust to fit your network */
const MAC: wire::EthernetAddress = EthernetAddress([0x20, 0x18, 0x03, 0x01, 0x00, 0x00]);
const IP: wire::Ipv4Address = Ipv4Address::new(192, 168, 10, 33);

/* Constants */
const KB: u16 = 1024; // bytes

#[entry]
fn main() -> ! {
    let dp = Peripherals::take().unwrap();
    let pins = dp.GPIO.split();
    let mut serial = dp
        .UART0
        .serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    writeln!(&mut serial, "\nstarting...").ok();

    let mut led = pins.gpio2.into_push_pull_output();
    let (mut timer1, _timer2) = dp.TIMER.timers();
    // turn the LED off during initialization
    led.set_high().ok();

    // SPI
    let spi = {
        let mut spi = dp.SPI1.spi(
            pins.gpio14.into_hspi(),
            pins.gpio12.into_hspi(),
            pins.gpio13.into_hspi(),
            SpiClock::Spi2MHz,
        );
        spi.set_data_mode(enc28j60::MODE);
        spi
    };

    timer1.delay_ms(100);

    // ENC28J60
    let enc28j60 = {
        let mut reset = pins.gpio5.into_push_pull_output();
        reset.set_high().unwrap();
        let int = pins.gpio4.into_pull_up_input();
        let mut cs = pins.gpio15.into_push_pull_output();
        cs.set_low().unwrap();

        Enc28j60::new(
            spi,
            OldOutputPin::new(cs),
            OldInputPin::new(int),
            Unconnected,
            &mut timer1,
            7 * KB,
            MAC.0,
        )
        .unwrap()
    };

    // LED on after initialization
    led.set_high().unwrap();

    // PHY Wrapper
    let mut rx_buf = [0u8; 1024];
    let mut tx_buf = [0u8; 1024];
    let mut eth = Phy::new(enc28j60, &mut rx_buf, &mut tx_buf);
    writeln!(serial, "eth initialized").unwrap();

    // Ethernet interface
    let local_addr = IP;
    let ip_addr = IpCidr::new(IpAddress::from(local_addr), 24);
    let mut ip_addrs = [ip_addr];
    let mut neighbor_storage = [None; 16];
    let neighbor_cache = NeighborCache::new(&mut neighbor_storage[..]);
    let mut iface = EthernetInterfaceBuilder::new(&mut eth)
        .ethernet_addr(MAC)
        .ip_addrs(&mut ip_addrs[..])
        .neighbor_cache(neighbor_cache)
        .finalize();
    writeln!(serial, "iface initialized").unwrap();

    // Sockets
    let mut server_rx_buffer = [0; 2048];
    let mut server_tx_buffer = [0; 2048];
    let server_socket = TcpSocket::new(
        TcpSocketBuffer::new(&mut server_rx_buffer[..]),
        TcpSocketBuffer::new(&mut server_tx_buffer[..]),
    );
    let mut sockets_storage = [None, None];
    let mut sockets = SocketSet::new(&mut sockets_storage[..]);
    let server_handle = sockets.add(server_socket);
    writeln!(serial, "sockets initialized").unwrap();

    // LED off after initialization
    led.set_high().unwrap();

    let mut count: u64 = 0;
    loop {
        match iface.poll(&mut sockets, Instant::from_millis(0)) {
            Ok(b) => {
                if b {
                    let mut socket = sockets.get::<TcpSocket>(server_handle);
                    if !socket.is_open() {
                        socket.listen(80).unwrap();
                    }

                    if socket.can_send() {
                        led.toggle().unwrap();
                        count += 1;

                        writeln!(serial, "tcp:80 send").unwrap();
                        write!(
                            socket,
                            "HTTP/1.1 200 OK\r\n\r\nHello!\nLED is currently {} and has been toggled {} times.\n",
                            match led.is_set_low().unwrap() {
                                true => "on",
                                false => "off",
                            },
                            count
                        )
                            .unwrap();

                        writeln!(serial, "tcp:80 close").unwrap();
                        socket.close();
                    }
                }
            }
            Err(e) => {
                writeln!(serial, "Error: {:?}", e).unwrap();
            }
        }
    }
}
