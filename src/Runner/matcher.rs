/// Returns the left-aligned substring of this expression which matches this pattern.
/// # Syntax
///   * `abc` matches the phrase "abc".
///   * `\?` matches any character.
///   * `\N` matches any alphanumeric character.
///   * `\A` matches any alphabetic character.
///   * `\0` matches any number.
///   * `\s` matches any whitespace.
///   * `\S` matches everything except whitespace.
///   * `\n` matches the return character.
///   * `\t` matches the tab character.
///   * `\*` matches the `*` character.
///   * `\\` matches the `\` character.
#[allow(dead_code)]
pub fn get_substring<'a>(expression : &'a str, pattern : &str, start : usize) -> Option<&'a str> {
    let mut end : usize = start;
    let mut i : usize = 0;
    for ch in expression.chars().skip(start) {
        let mut chars = pattern.chars();
        if let Some(pat) = chars.nth(i) {
            i += 1;
            if if pat == '\\' {
                // match command
                i += 1;
                match chars.next() {
                    Some('?') => false,
                    Some('N') => !ch.is_alphanumeric(),
                    Some('A') => !ch.is_alphabetic(),
                    Some('0') => !ch.is_numeric(),
                    Some('s') => ch != ' ' && ch != '\t' && ch != '\n' && ch != '\r',
                    Some('S') => ch == ' ' || ch == '\t' || ch == '\n' || ch == '\r',
                    Some('n') => ch != '\n',
                    Some('t') => ch != '\t',
                    Some('r') => ch != '\r',
                    Some('*') => ch != '*',
                    Some('\\') => ch != '\\',
                    _ => true
                }
            } else {
                // match text
                ch != pat
            } {
                // invalid
                return None;
            }
        } else {
            break;
        }
        end += 1;
    }
    if i < pattern.chars().count() {
        None
    } else {
        Some(&expression[start..end])
    }
}