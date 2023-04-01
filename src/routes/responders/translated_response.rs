use rocket::response::Responder;

use rocket::serde::Serialize;
use rocket::{Response, response, Request};

use rocket::http::{Header, ContentType};
use rocket::serde::json::Json;
pub struct TranslatedResponse<T : Serialize>{
    pub language : String,
    pub body :Json<T>
}

#[rocket::async_trait]
impl<'r, T: Serialize> Responder<'r, 'static> for TranslatedResponse<T> {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        Response::build_from(self.body.respond_to(&req).unwrap())
            .header(Header::new("Content-Language", self.language))
            .header(ContentType::JSON)
            .ok()
    }
}