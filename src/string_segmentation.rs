use std::str;

fn solution(s: &str) -> Vec<String> {
    let vec = s
        .chars()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .chunks(2)
        .map(|x| format!("{:_<2}", x.join("")))
        .collect();
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(solution("abcdef"), ["ab", "cd", "ef"]);
        assert_eq!(solution("abcdefg"), ["ab", "cd", "ef", "g_"]);
        assert_eq!(solution(""), [] as [&str; 0]);
    }
}
