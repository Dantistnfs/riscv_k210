#![no_main]
#![feature(asm)]
#![no_std]
#![feature(global_asm)]
use core::panic::PanicInfo;
use core::ops;
//use core::ptr;
use register::{mmio::*, register_bitfields};



const GPIO_BASE_ADDR : usize = 0x50200000;
const FPIOA_BASE_ADDR : usize = 0x502B0000;
const RTC_BASE_ADDR : usize = 0x50460000;
const SYSCTL_BASE_ADDR : usize = 0x50440000;


const FUNC_GPIO0: u8 = 24;
/*
>>> board_info.LED_R
14
>>> board_info.LED_B
12
>>> board_info.LED_G
13
*/


register_bitfields! {
    u32,
    UARTHS_DATA [
        data OFFSET(0)  NUMBITS(8)  [],
        zero OFFSET(8)  NUMBITS(23) [],
        full OFFSET(31) NUMBITS(1)  []
    ],
    UARTHS_TXCTRL [
        /* Bit 0 is txen, controls whether the Tx channel is active. */
        enable      OFFSET(0)  NUMBITS(1)  [
            Disabled = 0,
            Enabled  = 1],
        /* Bit 1 is nstop, 0 for one stop bit and 1 for two stop bits */
        stop        OFFSET(1)  NUMBITS(1)  [],
        /* Bits [15:2] is reserved */
        _reserved0  OFFSET(2)  NUMBITS(14) [],
        /* Bits [18:16] is threshold of interrupt triggers */
        cnt         OFFSET(16) NUMBITS(3)  [],
        /* Bits [31:19] is reserved */
        _reserved1  OFFSET(19) NUMBITS(13) []
    ],
    UARTHS_RXCTRL [
        /* Bit 0 is txen, controls whether the Rx channel is active. */
        enable      OFFSET(0)  NUMBITS(1)  [
            Disabled = 0,
            Enabled  = 1],
        /* Bits [15:1] is reserved */
        _reserved0  OFFSET(1)  NUMBITS(15) [],
        /* Bits [18:16] is threshold of interrupt triggers */
        cnt         OFFSET(16) NUMBITS(3)  [],
        /* Bits [31:19] is reserved */
        _reserved1  OFFSET(19) NUMBITS(13) []
    ],
    UARTHS_IEIP [
        /* Bit 0 is txwm, raised less than txcnt */
        txwm  OFFSET(0)  NUMBITS(1)  [],
        /* Bit 1 is rxwm, raised greater than rxcnt */
        rxwm  OFFSET(1)  NUMBITS(1)  [],
        /* Bits [31:2] is 0 */
        _zero OFFSET(2)  NUMBITS(30) []
    ],
    UARTHS_DIV [
        /* Bits [31:2] is baud rate divisor register */
        div   OFFSET(0)  NUMBITS(16) [],
        /* Bits [31:16] is 0 */
        _zero OFFSET(16) NUMBITS(16) []
    ]
}

const UARTHS_BASE_ADDR: usize = 0x3800_0000;

/* Register address offsets */
#[repr(C)]
pub struct UARTHS_REG{
    txfifo: ReadWrite<u32,  UARTHS_DATA::Register>, //0x00
    rxfifo: ReadWrite<u32,  UARTHS_DATA::Register>, //0x04
    txctrl: ReadWrite<u32,  UARTHS_TXCTRL::Register>, //0x08
    rxctrl: ReadWrite<u32,  UARTHS_RXCTRL::Register>, //0x0c
    ie: ReadWrite<u32,      UARTHS_IEIP::Register>, //0x10
    ip: ReadWrite<u32,      UARTHS_IEIP::Register>, //0x14
    div: ReadWrite<u32,     UARTHS_DIV::Register> //0x18

}

pub struct UARTHS;


/// Deref to UARTHS_REG
///
/// Allows writing
/// ```
/// self.IE.read()
/// ```
/// instead of something along the lines of
/// ```
/// unsafe { (*UARTHS::ptr()).IE.read() }
/// ```
impl ops::Deref for UARTHS {
    type Target = UARTHS_REG;

    fn deref(&self) -> &Self::Target {
        unsafe { &*Self::ptr() }
    }
}




impl UARTHS {
    #[no_mangle]
    pub fn new() -> UARTHS {
        UARTHS
    }

    /// Returns a pointer to the register block
    #[no_mangle]
    fn ptr() -> *const UARTHS_REG {
       UARTHS_BASE_ADDR as *const _
    }

    #[no_mangle]
    pub fn init(&self) {
        // initialize UART
        //let freq : u32 = 26_000_000; //I am not sure about freq, need to implemt pll registers and read them

        //let freq: u32 = 320_666_666; // around 90% that this is right freq
        //let div : u32 = freq / 115200 - 1;
        let div = 3700; //Rs
        //let div = 1700;

        self.div.write(UARTHS_DIV::div.val(div));
        self.txctrl.write(UARTHS_TXCTRL::enable::Enabled);
        self.rxctrl.write(UARTHS_RXCTRL::enable::Disabled);
        self.txctrl.write(UARTHS_TXCTRL::cnt.val(0));
        self.rxctrl.write(UARTHS_RXCTRL::cnt.val(0));
        self.ip.write(UARTHS_IEIP::txwm.val(1));
        self.ip.write(UARTHS_IEIP::rxwm.val(1));
        self.ie.write(UARTHS_IEIP::txwm.val(0));
        self.ie.write(UARTHS_IEIP::rxwm.val(1));
    }

    #[no_mangle]
    pub fn send(&self, c: char) {
        loop {
            if self.txfifo.read(UARTHS_DATA::full) == 0{
                break;
            }
            unsafe{asm!("nop")};
        }
        self.txfifo.write(UARTHS_DATA::data.val((c as u32))); //i know, it's very shitty design that will drop all bits over 8

    }

    #[no_mangle]
    pub fn puts(&self, string: &str) { //do not use or linking error will happen
        for c in string.chars() {
            // convert newline to carrige return + newline
            if c == '\n' {
                self.send('\r')
            }

            self.send(c);
        }
    }
}



global_asm!(include_str!("asm.S"));


#[no_mangle]
#[link_section = "_main_func"]
fn main_test() {
    let uart = UARTHS::new();

    uart.init();
    loop {
        uart.send('R');
        uart.send('u');
        uart.send('s');
        uart.send('t');
        uart.send('!');
        uart.send('\r');
        unsafe{asm!("nop")};
        //uart.send('\n');
    }
}




#[panic_handler]
#[inline(never)]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}
