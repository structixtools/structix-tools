/// Strip comments and collapse whitespace for clone comparison.
pub fn normalize(source: &str) -> String {
    let mut result = String::with_capacity(source.len());
    let mut chars = source.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '/' {
            match chars.peek() {
                Some('*') => {
                    // Block comment — consume until */
                    chars.next();
                    while let Some(c2) = chars.next() {
                        if c2 == '*' && chars.peek() == Some(&'/') {
                            chars.next();
                            break;
                        }
                    }
                    result.push(' ');
                }
                Some('/') => {
                    // Line comment — consume until newline
                    while let Some(&c2) = chars.peek() {
                        if c2 == '\n' {
                            break;
                        }
                        chars.next();
                    }
                }
                _ => result.push(c),
            }
        } else {
            result.push(c);
        }
    }

    // Collapse all whitespace runs to a single space and trim
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}
