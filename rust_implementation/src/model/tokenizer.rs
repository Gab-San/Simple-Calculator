use regex::Regex;

// No need for a package just for this function
pub fn build_tokenised_string(rgx: &Regex, haystack : &str) -> Vec<String> {
    Regex::find_iter(&rgx, haystack).map(|str_match| {String::from(str_match.as_str())}).collect()    
}


#[cfg(test)]
mod tests {
    use super::*;
    use regex::Regex;

    const RGX_STR : &str = r"\b[0-9]+\b|(?:\b|\B)[()*/+-](?:\b|\B)"; 

    #[test]
    fn tokenize_simple_expression() {
        let rgx  = Regex::new(RGX_STR).unwrap();
        let tokenised_string = build_tokenised_string(&rgx, "3+4/2");
        assert_eq!(tokenised_string, vec!["3", "+", "4", "/", "2"]);
    }

    #[test]
    fn tokenize_complex_expression() {
        let rgx  = Regex::new(RGX_STR).unwrap();
        let exp = "3+4*(2-5)/77-(82*(55-2))";
        let tokenised_string = build_tokenised_string(&rgx, exp);
        assert_eq!( tokenised_string, vec!["3", "+", "4", "*", "(", "2", "-", "5", ")", "/", "77",
            "-", "(", "82", "*", "(", "55", "-", "2", ")", ")",] );
    }
}

