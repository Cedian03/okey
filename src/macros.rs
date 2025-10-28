#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! error {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        ::defmt::error!($($arg)*)
    };
}

#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! warn {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        ::defmt::warn!($($arg)*)
    };
}

#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! info {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        ::defmt::info!($($arg)*)
    };
}

#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! debug {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        ::defmt::debug!($($arg)*)
    };
}

#[macro_export]
#[collapse_debuginfo(yes)]
macro_rules! trace {
    ($($arg:tt)*) => {
        #[cfg(feature = "defmt")]
        ::defmt::trace!($($arg)*)
    };
}
