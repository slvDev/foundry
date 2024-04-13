pub fn get_full_line(code: &str, start: usize) -> &str {
    let line_start = code[..start].rfind('\n').map_or(0, |index| index + 1);
    let line_end = code[start..].find('\n').map_or(code.len(), |index| start + index);
    &code[line_start..line_end]
}