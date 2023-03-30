use rocket::response::Responder;
use rocket::{Response, response, Request};
use rocket::http::{Header, ContentType};
use rocket::serde::json::Json;

use crate::bll::map_list::map_list;
use crate::bo::map::Map;

use super::guards::language_guard::Language;

pub struct TranslatedResponse{
    language : String,
    body :Json<Vec<Map>>
}

#[rocket::async_trait]
impl<'r> Responder<'r, 'static> for TranslatedResponse {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.body.respond_to(&req).unwrap())
            .header(Header::new("Content-Language", self.language))
            .header(ContentType::JSON)
            .ok()
    }
}

// @TODO deal with error 500
#[get("/map/list")]
pub async fn get_map_list(language: Language) -> TranslatedResponse {
    let json_response = map_list(String::from(&language.0)).await.unwrap();
    TranslatedResponse{
        body:json_response,
        language: language.0
    }
}