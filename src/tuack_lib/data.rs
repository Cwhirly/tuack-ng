use std::io;

use async_trait::async_trait;

/// 可读数据源——基础。
///
/// 只负责读取输入/答案字节，不关心数据来自文件系统还是内存。
#[async_trait]
pub trait Data: Send {
    /// 数据点编号
    fn id(&self) -> u32;

    /// 数据点输入
    async fn input(&self) -> io::Result<Vec<u8>>;

    /// 数据点输出（答案）
    async fn answer(&self) -> io::Result<Vec<u8>>;
}

/// 测试用数据——继承 [`Data`]，增加计分信息。
pub trait TestData: Data {
    /// 数据点得分
    fn score(&self) -> u32;

    /// 所属子任务编号
    fn subtask(&self) -> u32;
}
