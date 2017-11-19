// use a busy-wait to implement fade.
//
// TODO:
// Use the individual pin instead of changing the whole port
// The math depends on overruns, which is unintuitive.
// There is lots of unsafe to wrap in helper functions

#![feature(asm, lang_items, unwind_attributes)]

#![no_std]
#![no_main]

extern crate arduino;
extern crate avr_delay;

use arduino::{DDRB, PORTB};
use core::ptr::write_volatile;
use avr_delay::{delay_us};

#[no_mangle]
pub extern fn main() {
    let mut out: u8 = 0x00;
    let mut fade_index: u32 = 1;
    let mut fade_direction: i32 = 1;
    unsafe { write_volatile(DDRB, 0xff) }

    loop {
        if fade_index == 1024 || fade_index == 0 {
            fade_direction = -fade_direction;
        }
        // toggle the port with xor
        out = out ^ 0xff;
        // write the value
        unsafe { write_volatile(PORTB, out) }
        // delay for a period
        delay_us(fade_index);
        out = out ^ 0xff;
        unsafe { write_volatile(PORTB, out) }
        // delay for the oposite period as above.
        delay_us(1024 - fade_index);
        // This depends on whacky conversion and overrun behavior
        fade_index += fade_direction as u32;
    }
}

// std stubs. A PR in the arduino project may eliminate the need for this.
pub mod std {
    #[lang = "eh_personality"]
    #[no_mangle]
    pub unsafe extern "C" fn rust_eh_personality(_state: (), _exception_object: *mut (), _context: *mut ()) -> () {
    }

    #[lang = "panic_fmt"]
    #[unwind]
    pub extern fn rust_begin_panic(_msg: (), _file: &'static str, _line: u32) -> ! {
        loop { }
    }
}

