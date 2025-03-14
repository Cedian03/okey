// Two variants of debouncing, both implemented based on this paper: https://my.eng.utah.edu/~cs5780/debouncing.pdf.

mod counter;
mod simple;

use crate::usb::SCAN_INTERVAL;

pub use counter::Counter;
pub use simple::Simple;

use super::Scan;

const DEBOUNCE: u16 = 5; // ms
const DEBOUNCE_COUNT: u16 = DEBOUNCE / (SCAN_INTERVAL as u16);

pub fn debounce<const W: usize, const H: usize>(scanner: impl Scan<W, H>) -> impl Scan<W, H> {
    Simple::new(scanner)
} 