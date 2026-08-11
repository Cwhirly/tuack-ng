use std::io;

use async_trait::async_trait;
use tokio::io::AsyncRead;

/// 统一的异步可读流抽象：`AsyncRead + Unpin + Send` 的便捷 trait
pub trait AsyncReader: AsyncRead + Unpin + Send {}

impl<T: AsyncRead + Unpin + Send> AsyncReader for T {}

/// 可读数据源
#[async_trait]
pub trait Data: Send {
    /// 数据点输入（返回流，不读入内存）
    async fn input(&self) -> io::Result<Box<dyn AsyncReader>>;

    /// 数据点输出（答案，返回流）
    async fn answer(&self) -> io::Result<Box<dyn AsyncReader>>;
}
