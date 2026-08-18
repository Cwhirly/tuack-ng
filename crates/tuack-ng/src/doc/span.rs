//! span（字节区间）-> 行列 换算工具。

/// 根据源码计算字节偏移对应的行号（1 起）与列号（1 起）。
pub fn offset_to_line_col(source: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut line_start = 0usize;
    for (i, b) in source.bytes().enumerate() {
        if i >= offset {
            break;
        }
        if b == b'\n' {
            line += 1;
            line_start = i + 1;
        }
    }
    // 列按字符数（Unicode 字符）计。
    let col = source[line_start..offset.min(source.len())].chars().count() + 1;
    (line, col)
}

/// 将 `Option<Span>` 转换为 `(Option<line>, Option<col>)`。
pub fn span_to_line_col(
    source: &str,
    span: Option<tuack_ng_parser::Span>,
) -> (Option<usize>, Option<usize>) {
    match span {
        Some(s) => {
            let (line, col) = offset_to_line_col(source, s.start);
            (Some(line), Some(col))
        }
        None => (None, None),
    }
}
