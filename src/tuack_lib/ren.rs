//! 渲染抽象。
//!
//! 渲染指：以数据变换为核心的操作，旨在将原文档转换为各种形式。
//!
//! - `RenderDocument` 是不可变输入，`Renderer::render` 产出 `Vec<OutputFile>`。
//! - 渲染器允许的，可忽略不计的副作用：写自己的临时目录、调用外部命令（如 typst）。
//! - 渲染器禁止：访问 `gctx()`、直接读取用户资源（除非经 `AssetProvider`）、写最终输出目录。

pub mod asset;
pub mod document;
pub mod output;

pub use asset::AssetProvider;
pub use document::{
    DateInfo, Problem, ProblemMeta, ProblemType, RenConfig, RenderDocument, SupportLanguage,
};
pub use output::OutputFile;

use crate::prelude::*;

/// 渲染器：`RenderDocument -> (主产物相对路径，产物文件列表)`
#[async_trait]
pub trait Renderer: Send + Sync {
    async fn render(&self, doc: &RenderDocument) -> Result<(PathBuf, Vec<OutputFile>)>;
}
