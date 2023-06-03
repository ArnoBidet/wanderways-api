use rocket::request::{FromRequest, Outcome, Request};

use crate::routes::utils::lang_utils::{DEFAULT_LANGUAGE, get_lang};

#[derive(Debug)]
pub struct Language(pub String);

// cannot be easily tested because mocking Request is tremendously hard, moreover it's mostly 3rd party code
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Language {
    type Error = ();
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        // Returns the language if given. If no pattern recognized, returns en-US as default.
        match req.headers().get_one("Accept-Language") {
            None => Outcome::Success(Language(String::from(DEFAULT_LANGUAGE))),
            Some(lang) => Outcome::Success(Language(get_lang(String::from(lang)))),
        }
    }
}