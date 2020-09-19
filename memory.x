/* This memory map assumes the flash cache is on; 
   the blocks used are excluded from the various memory ranges 
   
   see: https://github.com/espressif/esp-idf/blob/master/components/soc/src/esp32/soc_memory_layout.c
   for details
   */

/* override entry point */
ENTRY(ESP8266Reset)

/* define stack size for both cores */
STACK_SIZE = 8k;

/* Specify main memory areas */
MEMORY
{
  reserved_cache_seg     : ORIGIN = 0x40070000, len = 64k /* SRAM0; reserved for usage as flash cache*/
  vectors_seg ( RX )     : ORIGIN = 0x40100000, len =  0x100 /* SRAM0 */
  iram_seg ( RX )        : ORIGIN = 0x40100100, len = 0x8000 - 0x0100 /* SRAM0 */

  reserved_for_rom_seg   : ORIGIN = 0x3FFAE000, len = 8k /* SRAM2; reserved for usage by the ROM */
  dram_seg ( RW )        : ORIGIN = 0x3FFE8000, len = 0x14000 /* SRAM2+1; first 64kB used by BT if enable */
  reserved_for_boot_seg  : ORIGIN = 0x3FFDC200, len = 144k /* SRAM1; reserved for static ROM usage; can be used for heap */

  /* external flash 
     The 0x20 offset is a convenience for the app binary image generation.
     Flash cache has 64KB pages. The .bin file which is flashed to the chip
     has a 0x18 byte file header, and each segment has a 0x08 byte segment
     header. Setting this offset makes it simple to meet the flash cache MMU's
     constraint that (paddr % 64KB == vaddr % 64KB).)
  */
  irom_seg ( RX )        : ORIGIN = 0x400D0020, len = 3M - 0x20
  drom_seg ( R )         : ORIGIN = 0x3F400020, len = 4M - 0x20
}

/* map generic regions to output sections */
INCLUDE "alias.x"

_heap_end = ABSOLUTE(ORIGIN(dram_seg))+LENGTH(dram_seg)+LENGTH(reserved_for_boot_seg) - 2*STACK_SIZE;
_text_heap_end = ABSOLUTE(ORIGIN(iram_seg)+LENGTH(iram_seg));

_stack_start_cpu1 = _heap_end;
_stack_end_cpu1 = _stack_start_cpu1 + STACK_SIZE;
_stack_start_cpu0 = _stack_end_cpu1;
_stack_end_cpu0 = _stack_start_cpu0 + STACK_SIZE;

EXTERN(DefaultHandler);

PROVIDE(__slc_interrupt = __default_interrupt);
PROVIDE(__spi_interrupt = __default_interrupt);
PROVIDE(__gpio_interrupt = __default_interrupt);
PROVIDE(__uart_interrupt = __default_interrupt);
PROVIDE(__ccompare_interrupt = __default_interrupt);
PROVIDE(__soft_interrupt = __default_interrupt);
PROVIDE(__wdt_interrupt = __default_interrupt);
PROVIDE(__timer1_interrupt = __default_interrupt);