use std::io;

use async_trait::async_trait;

/// 只读的数据点
#[async_trait]
pub trait Data: Send {
    /// 数据点输入
    async fn input(&self) -> io::Result<Vec<u8>>;

    /// 数据点输出（答案）
    async fn answer(&self) -> io::Result<Vec<u8>>;
}
