pub trait TranslationParser {
    fn translation_parser(&self) -> Vec<String>;
}

impl TranslationParser for String{
    fn translation_parser(&self) -> Vec<String>{
        self.split("||").into_iter().map(|el|String::from(el)).collect::<Vec<String>>()
    }
}