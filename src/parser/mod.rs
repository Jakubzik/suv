pub(crate) mod date_calculation;
pub(crate) mod parse_date;
pub fn between<'a>(source: &'a str, start: &'a str, end: &'a str) -> &'a str {
    let opt_start_position = source.find(start);
    if let Some(is_start_position) = opt_start_position {
        let start_position = is_start_position + start.len();
        let source = &source[start_position..];
        let end_position = source.find(end).unwrap_or_default();
        return &source[..end_position];
    }
    ""
}
