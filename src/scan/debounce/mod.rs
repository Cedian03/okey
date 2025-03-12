// Two variants of debouncing, both implemented based on this paper: https://my.eng.utah.edu/~cs5780/debouncing.pdf.

mod counter;
mod simple;

use crate::usb::SCAN_INTERVAL;

pub use counter::Counter;
pub use simple::Simple;

const DEBOUNCE: u16 = 5; // ms
const DEBOUNCE_COUNT: u16 = DEBOUNCE / (SCAN_INTERVAL as u16);