pub fn errors(s: &str) -> Result<Vec<char>, char> {
    s.chars()
        .try_fold(Vec::new(), |mut acc, c| {
            match c {
                '(' => {
                    acc.push(')');
                }
                '{' => {
                    acc.push('}');
                }
                '[' => {
                    acc.push(']');
                }
                '<' => {
                    acc.push('>');
                }
                ')' | '}' | ']' | '>' => match acc.pop() {
                    Some(o) if o == c => (),
                    Some(o) if o != c => return Err(c),
                    Some(_) => unreachable!(),
                    None => return Err(c),
                },
                _ => unreachable!(),
            };
            Ok(acc)
        })
        .map(|mut v| {
            v.reverse();
            v
        })
}
