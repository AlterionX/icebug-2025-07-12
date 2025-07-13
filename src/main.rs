use std::collections::HashMap;

use axum::{http::header::SET_COOKIE, response::{AppendHeaders, IntoResponse, Response}};
use chrono::Utc;
use cookie::{Cookie, SameSite};

#[derive(Debug)]
pub struct S {
}

impl IntoResponse for S {
    fn into_response(self) -> Response {
        let s = Cookie::build(("__Host-a", "a".to_owned()))
            .path("/")
            .expires(Utc::now().into())
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .to_string();

        (
            AppendHeaders([
                (SET_COOKIE, s)
            ]),
            HashMap::new(),
        ).into_response()
    }
}

fn main() {
}
