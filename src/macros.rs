#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        $crate::defmt::error!($($arg)*)
    };
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        $crate::defmt::warn!($($arg)*)
    };
}

#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        $crate::defmt::info!($($arg)*)
    };
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        $crate::defmt::debug!($($arg)*)
    };
}

#[macro_export]
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        $crate::defmt::trace!($($arg)*)
    };
}
