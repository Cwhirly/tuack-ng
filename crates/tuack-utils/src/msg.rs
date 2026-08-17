//! 消息输出接口：主 crate 在 init 时接入真实实现，未接入时回退到 log。

use anyhow::{Result, anyhow};
use std::sync::OnceLock;

/// 消息级别
#[derive(Clone, Copy, PartialEq)]
pub enum Level {
    Debug,
    Info,
    Warn,
    Error,
}

type Printer = Box<dyn Fn(Level, String) + Send + Sync>;

static PRINTER: OnceLock<Printer> = OnceLock::new();

/// 接入消息输出实现（主 crate init 时调用）。
pub fn install(printer: Printer) -> Result<()> {
    PRINTER
        .set(printer)
        .map_err(|_| anyhow!("消息输出已接入"))
}

/// 打印一条消息；未接入时回退到 log。
pub fn print(level: Level, msg: String) {
    if let Some(printer) = PRINTER.get() {
        printer(level, msg);
    } else {
        match level {
            Level::Debug => log::debug!("{}", msg),
            Level::Info => log::info!("{}", msg),
            Level::Warn => log::warn!("{}", msg),
            Level::Error => log::error!("{}", msg),
        }
    }
}

pub fn debug(msg: String) {
    print(Level::Debug, msg);
}

pub fn info(msg: String) {
    print(Level::Info, msg);
}

pub fn warn(msg: String) {
    print(Level::Warn, msg);
}

pub fn error(msg: String) {
    print(Level::Error, msg);
}
