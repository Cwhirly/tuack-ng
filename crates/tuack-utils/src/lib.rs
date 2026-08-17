pub mod assets;
pub mod checkers;
pub mod command;
pub mod compilers;
pub mod data;
pub mod dump;
pub mod msg;
pub mod prelude;
pub mod process;
pub mod ren;
pub mod validators;

#[macro_export]
macro_rules! msg_warn {
    ($($arg:tt)*) => {
        $crate::msg::warn(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! msg_info {
    ($($arg:tt)*) => {
        $crate::msg::info(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! msg_error {
    ($($arg:tt)*) => {
        $crate::msg::error(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! msg_debug {
    ($($arg:tt)*) => {
        $crate::msg::debug(format!($($arg)*))
    };
}
