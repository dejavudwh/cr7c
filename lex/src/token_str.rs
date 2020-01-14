use std::fmt;
use std::cmp::Eq;

pub struct TokenStr {
    chars: Vec<char>,
}

impl TokenStr {
    pub fn new(chars: Vec<char>) -> TokenStr {
        TokenStr {
            chars,
        }
    }
}

impl fmt::Display for TokenStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s: String = self.chars.iter().collect();
        write!(f, "{}", s)
    }
}

impl PartialEq for TokenStr {
    fn eq(&self, other: &Self) -> bool {
        let len = self.chars.len();

        for i in 0..len {
            if self.chars[i] != other.chars[i] {
                return false
            } else {
                continue
            }
        }

        return true
    }
}

impl Eq for TokenStr {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_streq() {
        let v1: Vec<char> = vec!['a', 'b', 'c'];
        let v2: Vec<char> = vec!['a', 'b', 'c'];

        let t1 = TokenStr::new(v1);
        let t2 = TokenStr::new(v2);

        assert!(t1 == t2);
    }
}