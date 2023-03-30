use regex::Regex;
use rocket::request::{FromRequest, Outcome, Request};

#[derive(Debug)]
pub struct Language(pub String);

const DEFAULT_LANGUAGE: &str = "en-US";


fn is_valid_language_code(language_code: &str) -> String {
    if Regex::new(r"fr(-FR)?").unwrap().is_match(language_code) {
        return String::from("fr-FR");
    }
    if Regex::new(r"en(-US)?").unwrap().is_match(language_code) {
        return String::from("en-US");
    }
    if Regex::new(r"de(-DE)?").unwrap().is_match(language_code) {
        return String::from("de-DE");
    }
    if Regex::new(r"es(-ES)?").unwrap().is_match(language_code) {
        return String::from("es-ES");
    }
    if Regex::new(r"pt(-PT)?").unwrap().is_match(language_code) {
        return String::from("pt-PT");
    }
    String::from("en-US")
}

#[cfg(test)]
mod is_valid_language_code_tests {
    use crate::routes::guards::language_guard::is_valid_language_code;
    #[test]
    fn correct_short() {
        assert_eq!(is_valid_language_code("fr"), String::from("fr-FR"))
    }
    #[test]
    fn correct() {
        assert_eq!(is_valid_language_code("fr-FR"), String::from("fr-FR"))
    }
    #[test]
    fn incorrect() {
        assert_ne!(is_valid_language_code("fr-FR"), String::from("fr-fR"))
    }
    #[test]
    fn default() {
        assert_eq!(is_valid_language_code(""), String::from("en-US"))
    }
}

fn get_lang(lang: &str) -> String {
    // not doing : [a-z]{2}(-[A-Z]{2})? because the second part is useless and impacts performance
    let re = Regex::new(r"[a-z]{2}").unwrap();
    match re.find(lang) {
        Some(matched) => is_valid_language_code(matched.as_str()),
        None => String::from(DEFAULT_LANGUAGE),
    }
}

#[cfg(test)]
mod get_lang_tests {
    use crate::routes::guards::language_guard::get_lang;
    #[test]
    fn short() {
        assert_eq!(get_lang("fr"), String::from("fr-FR"))
    }
    #[test]
    fn one() {
        assert_eq!(get_lang("fr-FR"), String::from("fr-FR"))
    }
    #[test]
    fn multiple() {
        assert_eq!(get_lang("fr-FRde-DE"), String::from("fr-FR"))
    }
    #[test]
    fn first_from_multiple() {
        assert_ne!(get_lang("de-DEfr-FR"), String::from("fr-FR"))
    }
    #[test]
    fn default() {
        assert_eq!(get_lang(""), String::from("en-US"))
    }
}
