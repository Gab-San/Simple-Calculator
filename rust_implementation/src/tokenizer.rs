use std::slice::Iter;
use regex::Regex;

pub struct StringTokenizer {
    tokenised_string : Vec<String>,
}

impl StringTokenizer {
    pub fn new(rgx : Regex, haystack : &str) -> Self {
        let tokenised_string : Vec<String> = Regex::find_iter(&rgx, haystack).map(|str_match| {String::from(str_match.as_str())}).collect();
        Self {
            tokenised_string,
        }
    }

    pub fn iter(&self) -> Iter<'_, String>   {
        self.tokenised_string.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    #[test]
    fn tokenize_simple_expression() {
        let rgx  = Regex::new(r"\b[0-9]+\b|(?:\b|\B)[()*/+-](?:\b|\B)").unwrap();
        let tokenizer = StringTokenizer::new(rgx, "3+4/2");
        assert_eq!(tokenizer.tokenised_string, vec!["3", "+", "4", "/", "2"]);
    }

    #[test]
    fn tokenize_complex_expression() {
        let rgx  = Regex::new(r"\b[0-9]+\b|(?:\b|\B)[()*/+-](?:\b|\B)").unwrap();
        let exp = "3+4*(2-5)/77-(82*(55-2))";
        let tokenizer = StringTokenizer::new(rgx, exp);
        assert_eq!(tokenizer.tokenised_string, vec![
            "3",
            "+",
            "4",
            "*",
            "(",
            "2",
            "-",
            "5",
            ")",
            "/",
            "77",
            "-",
            "(",
            "82",
            "*",
            "(",
            "55",
            "-",
            "2",
            ")",
            ")",
        ])
    }
}

