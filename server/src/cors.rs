use rocket::http::Method;
use rocket::response::Responder;
use rocket::{
    fairing::{Fairing, Info, Kind},
    Request, Response,
};
use std::io::Cursor;

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Origin",
            "*",
        ));
        response.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, OPTIONS, PUT, DELETE",
        ));
        response.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));
        if request.method() == Method::Options {
            response.set_sized_body(0, Cursor::new(""));
        }
    }
}
