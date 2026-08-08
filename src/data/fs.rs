use std::io;
use std::path::PathBuf;

use async_trait::async_trait;

use crate::config::{ExpandedDataItem, ExpandedSampleItem};
use crate::tuack_lib::data::{Data, TestData};

/// 被包装的数据来源引用
enum TestItemRef<'a> {
    Data(&'a ExpandedDataItem),
    Sample(&'a ExpandedSampleItem),
}

/// 从文件系统读取的测试数据。
///
/// 持有基础目录与对配置项的引用，不拷贝数据，也不修改配置结构。
pub struct FsTestData<'a> {
    base_dir: PathBuf,
    item: TestItemRef<'a>,
}

impl<'a> FsTestData<'a> {
    pub fn from_data(base_dir: PathBuf, item: &'a ExpandedDataItem) -> Self {
        Self {
            base_dir,
            item: TestItemRef::Data(item),
        }
    }

    pub fn from_sample(base_dir: PathBuf, item: &'a ExpandedSampleItem) -> Self {
        Self {
            base_dir,
            item: TestItemRef::Sample(item),
        }
    }

    fn input_name(&self) -> &str {
        match &self.item {
            TestItemRef::Data(item) => &item.input,
            TestItemRef::Sample(item) => &item.input,
        }
    }

    fn answer_name(&self) -> &str {
        match &self.item {
            TestItemRef::Data(item) => &item.output,
            TestItemRef::Sample(item) => &item.output,
        }
    }
}

#[async_trait]
impl Data for FsTestData<'_> {
    fn id(&self) -> u32 {
        match &self.item {
            TestItemRef::Data(item) => item.id,
            TestItemRef::Sample(item) => item.id,
        }
    }

    async fn input(&self) -> io::Result<Vec<u8>> {
        tokio::fs::read(self.base_dir.join(self.input_name())).await
    }

    async fn answer(&self) -> io::Result<Vec<u8>> {
        tokio::fs::read(self.base_dir.join(self.answer_name())).await
    }
}

impl TestData for FsTestData<'_> {
    fn score(&self) -> u32 {
        match &self.item {
            TestItemRef::Data(item) => item.score,
            TestItemRef::Sample(_) => 1,
        }
    }

    fn subtask(&self) -> u32 {
        match &self.item {
            TestItemRef::Data(item) => item.subtask,
            TestItemRef::Sample(_) => 0,
        }
    }
}
