#![doc = "Peripheral access API for ESP8266 microcontrollers (generated using svd2rust v0.31.2 ( ))\n\nYou can find an overview of the generated API [here].\n\nAPI features to be included in the [next] svd2rust release can be generated by cloning the svd2rust [repository], checking out the above commit, and running `cargo doc --open`.\n\n[here]: https://docs.rs/svd2rust/0.31.2/svd2rust/#peripheral-api\n[next]: https://github.com/rust-embedded/svd2rust/blob/master/CHANGELOG.md#unreleased\n[repository]: https://github.com/rust-embedded/svd2rust"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![doc(html_logo_url = "https://avatars.githubusercontent.com/u/46717278")]
#![no_std]
use core::marker::PhantomData;
use core::ops::Deref;
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 3;
#[allow(unused_imports)]
use generic::*;
#[doc = r"Common register and bit access and modify traits"]
pub mod generic;
#[cfg(feature = "rt")]
extern "C" {}
#[doc(hidden)]
#[repr(C)]
pub union Vector {
    pub _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
pub static __INTERRUPTS: [Vector; 0] = [];
#[doc = r"Enumeration of all the interrupts."]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {}
unsafe impl xtensa_lx::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        match self {}
    }
}
#[doc = r" TryFromInterruptError"]
#[derive(Debug, Copy, Clone)]
pub struct TryFromInterruptError(());
impl Interrupt {
    #[doc = r" Attempt to convert a given value into an `Interrupt`"]
    #[inline]
    pub fn try_from(value: u16) -> Result<Self, TryFromInterruptError> {
        match value {
            _ => Err(TryFromInterruptError(())),
        }
    }
}
#[doc = "DPORT"]
pub struct DPORT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for DPORT {}
impl DPORT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const dport::RegisterBlock = 0x3ff0_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const dport::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for DPORT {
    type Target = dport::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for DPORT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DPORT").finish()
    }
}
#[doc = "DPORT"]
pub mod dport;
#[doc = "EFUSE"]
pub struct EFUSE {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for EFUSE {}
impl EFUSE {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const efuse::RegisterBlock = 0x3ff0_0050 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const efuse::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for EFUSE {
    type Target = efuse::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for EFUSE {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE").finish()
    }
}
#[doc = "EFUSE"]
pub mod efuse;
#[doc = "GPIO"]
pub struct GPIO {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for GPIO {}
impl GPIO {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const gpio::RegisterBlock = 0x6000_0300 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const gpio::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for GPIO {
    type Target = gpio::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for GPIO {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GPIO").finish()
    }
}
#[doc = "GPIO"]
pub mod gpio;
#[doc = "I2S"]
pub struct I2S {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for I2S {}
impl I2S {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const i2s::RegisterBlock = 0x6000_0e00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const i2s::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for I2S {
    type Target = i2s::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for I2S {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("I2S").finish()
    }
}
#[doc = "I2S"]
pub mod i2s;
#[doc = "IO_MUX"]
pub struct IO_MUX {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for IO_MUX {}
impl IO_MUX {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const io_mux::RegisterBlock = 0x6000_0800 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const io_mux::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for IO_MUX {
    type Target = io_mux::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for IO_MUX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_MUX").finish()
    }
}
#[doc = "IO_MUX"]
pub mod io_mux;
#[doc = "RTC"]
pub struct RTC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTC {}
impl RTC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtc::RegisterBlock = 0x6000_0700 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtc::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for RTC {
    type Target = rtc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTC").finish()
    }
}
#[doc = "RTC"]
pub mod rtc;
#[doc = "SLC"]
pub struct SLC {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SLC {}
impl SLC {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const slc::RegisterBlock = 0x6000_0b00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const slc::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for SLC {
    type Target = slc::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SLC {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLC").finish()
    }
}
#[doc = "SLC"]
pub mod slc;
#[doc = "SPI0"]
pub struct SPI0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI0 {}
impl SPI0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi0::RegisterBlock = 0x6000_0200 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi0::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for SPI0 {
    type Target = spi0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI0").finish()
    }
}
#[doc = "SPI0"]
pub mod spi0;
#[doc = "SPI1"]
pub struct SPI1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for SPI1 {}
impl SPI1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const spi1::RegisterBlock = 0x6000_0100 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const spi1::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for SPI1 {
    type Target = spi1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for SPI1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI1").finish()
    }
}
#[doc = "SPI1"]
pub mod spi1;
#[doc = "TIMER"]
pub struct TIMER {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for TIMER {}
impl TIMER {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const timer::RegisterBlock = 0x6000_0600 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const timer::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for TIMER {
    type Target = timer::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for TIMER {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER").finish()
    }
}
#[doc = "TIMER"]
pub mod timer;
#[doc = "UART0"]
pub struct UART0 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART0 {}
impl UART0 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart0::RegisterBlock = 0x6000_0000 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart0::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for UART0 {
    type Target = uart0::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART0").finish()
    }
}
#[doc = "UART0"]
pub mod uart0;
#[doc = "UART1"]
pub struct UART1 {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for UART1 {}
impl UART1 {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const uart1::RegisterBlock = 0x6000_0f00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const uart1::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for UART1 {
    type Target = uart1::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for UART1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UART1").finish()
    }
}
#[doc = "UART1"]
pub mod uart1;
#[doc = "WDT"]
pub struct WDT {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for WDT {}
impl WDT {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const wdt::RegisterBlock = 0x6000_0900 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const wdt::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for WDT {
    type Target = wdt::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for WDT {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDT").finish()
    }
}
#[doc = "WDT"]
pub mod wdt;
#[doc = "RNG register"]
pub struct RNG {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RNG {}
impl RNG {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rng::RegisterBlock = 0x3ff2_0e44 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rng::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for RNG {
    type Target = rng::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RNG {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RNG").finish()
    }
}
#[doc = "RNG register"]
pub mod rng;
#[doc = "Internal I2C registers"]
pub struct RTCCNTL {
    _marker: PhantomData<*const ()>,
}
unsafe impl Send for RTCCNTL {}
impl RTCCNTL {
    #[doc = r"Pointer to the register block"]
    pub const PTR: *const rtccntl::RegisterBlock = 0x6000_0d00 as *const _;
    #[doc = r"Return the pointer to the register block"]
    #[inline(always)]
    pub const fn ptr() -> *const rtccntl::RegisterBlock {
        Self::PTR
    }
    #[doc = r" Steal an instance of this peripheral"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Ensure that the new instance of the peripheral cannot be used in a way"]
    #[doc = r" that may race with any existing instances, for example by only"]
    #[doc = r" accessing read-only or write-only registers, or by consuming the"]
    #[doc = r" original peripheral and using critical sections to coordinate"]
    #[doc = r" access between multiple new instances."]
    #[doc = r""]
    #[doc = r" Additionally, other software such as HALs may rely on only one"]
    #[doc = r" peripheral instance existing to ensure memory safety; ensure"]
    #[doc = r" no stolen instances are passed to such software."]
    pub unsafe fn steal() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}
impl Deref for RTCCNTL {
    type Target = rtccntl::RegisterBlock;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::PTR }
    }
}
impl core::fmt::Debug for RTCCNTL {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RTCCNTL").finish()
    }
}
#[doc = "Internal I2C registers"]
pub mod rtccntl;
#[no_mangle]
static mut DEVICE_PERIPHERALS: bool = false;
#[doc = r" All the peripherals."]
#[allow(non_snake_case)]
pub struct Peripherals {
    #[doc = "DPORT"]
    pub DPORT: DPORT,
    #[doc = "EFUSE"]
    pub EFUSE: EFUSE,
    #[doc = "GPIO"]
    pub GPIO: GPIO,
    #[doc = "I2S"]
    pub I2S: I2S,
    #[doc = "IO_MUX"]
    pub IO_MUX: IO_MUX,
    #[doc = "RTC"]
    pub RTC: RTC,
    #[doc = "SLC"]
    pub SLC: SLC,
    #[doc = "SPI0"]
    pub SPI0: SPI0,
    #[doc = "SPI1"]
    pub SPI1: SPI1,
    #[doc = "TIMER"]
    pub TIMER: TIMER,
    #[doc = "UART0"]
    pub UART0: UART0,
    #[doc = "UART1"]
    pub UART1: UART1,
    #[doc = "WDT"]
    pub WDT: WDT,
    #[doc = "RNG"]
    pub RNG: RNG,
    #[doc = "RTCCNTL"]
    pub RTCCNTL: RTCCNTL,
}
impl Peripherals {
    #[doc = r" Returns all the peripherals *once*."]
    #[cfg(feature = "critical-section")]
    #[inline]
    pub fn take() -> Option<Self> {
        critical_section::with(|_| {
            if unsafe { DEVICE_PERIPHERALS } {
                return None;
            }
            Some(unsafe { Peripherals::steal() })
        })
    }
    #[doc = r" Unchecked version of `Peripherals::take`."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Each of the returned peripherals must be used at most once."]
    #[inline]
    pub unsafe fn steal() -> Self {
        DEVICE_PERIPHERALS = true;
        Peripherals {
            DPORT: DPORT {
                _marker: PhantomData,
            },
            EFUSE: EFUSE {
                _marker: PhantomData,
            },
            GPIO: GPIO {
                _marker: PhantomData,
            },
            I2S: I2S {
                _marker: PhantomData,
            },
            IO_MUX: IO_MUX {
                _marker: PhantomData,
            },
            RTC: RTC {
                _marker: PhantomData,
            },
            SLC: SLC {
                _marker: PhantomData,
            },
            SPI0: SPI0 {
                _marker: PhantomData,
            },
            SPI1: SPI1 {
                _marker: PhantomData,
            },
            TIMER: TIMER {
                _marker: PhantomData,
            },
            UART0: UART0 {
                _marker: PhantomData,
            },
            UART1: UART1 {
                _marker: PhantomData,
            },
            WDT: WDT {
                _marker: PhantomData,
            },
            RNG: RNG {
                _marker: PhantomData,
            },
            RTCCNTL: RTCCNTL {
                _marker: PhantomData,
            },
        }
    }
}