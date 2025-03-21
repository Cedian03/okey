// Two variants of debouncing, both implemented based on this paper: https://my.eng.utah.edu/~cs5780/debouncing.pdf.

mod counter;
mod simple;

use embassy_time::Duration;

use crate::SCAN_INTERVAL;

use super::Scan;

pub use counter::Counter;
pub use simple::Simple;

const DEBOUNCE: Duration = Duration::from_millis(5);
const DEBOUNCE_COUNT: u64 = DEBOUNCE.as_ticks() / SCAN_INTERVAL.as_ticks();

pub const fn debounce<const W: usize, const H: usize>(scanner: impl Scan<W, H>) -> impl Scan<W, H> {
    Simple::new(scanner)
} 