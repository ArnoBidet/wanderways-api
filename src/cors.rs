use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct CORS;

const ALLOWED_DOMAINS : &'static [&str] = &["https://wanderways.io", "https://doc.wanderways.io"];

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        let acao = ALLOWED_DOMAINS.iter().find(|domain|{
            request.headers().get_one("Origin").unwrap_or("https://wanderways.io").to_string() == domain.to_string()
        }).unwrap_or(&"https://wanderways.io");
        response.set_header(Header::new("Access-Control-Allow-Origin", *acao)); // possible vulnerability issue by stack overflow ?
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
