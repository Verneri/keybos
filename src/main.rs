#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![feature(global_asm)]
#![feature(asm)]
#![feature(maybe_uninit)]
#![feature(range_contains)]



#[cfg(not(test))]
extern crate panic_abort;

mod registers;

mod gpio;

mod aux;


use aux::Uart;
use core::fmt::Write;

static PBASE:u64 =  0x3F000000;

const BAUD_RATE:u32 = 115200;

#[no_mangle]
pub extern "C" fn  kernel_main() -> ! {
    let mut uart = Uart::new(PBASE);
    uart.init(BAUD_RATE);
    write!(uart,"hello").unwrap();
    writeln!(uart,"Hello, world!").expect("write failed");
    let el = registers::read_el();
    writeln!(uart, "Exception Level {}", el).expect("write failed");
    loop {
        let b = uart.recv();
        uart.send(b);
    }
    format_args_nl!("ldskld")
}











#[cfg(not(test))]
global_asm!(include_str!("boot.S"));
