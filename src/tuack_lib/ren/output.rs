use std::path::PathBuf;

use crate::tuack_lib::data::AsyncReader;

/// 渲染/导出产物文件：相对路径 + 字节流。
pub struct OutputFile {
    /// 相对路径，如 `img/a.png`、`main.typ`
    pub path: PathBuf,
    pub bytes: Box<dyn AsyncReader>,
}
