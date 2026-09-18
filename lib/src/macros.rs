macro_rules! debug_log {
    ($($arg:tt)*) => {
        #[cfg(feature = "log")]
        log::debug!($($arg)*)
    };
}
