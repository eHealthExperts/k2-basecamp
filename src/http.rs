extern crate hyper;
extern crate log;
extern crate serde;
extern crate serde_json;

use hyper::Client;
use hyper::Error;
use hyper::client::response::Response;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde::Serialize;
use std::env::var;

const BASE_URL: &'static str = "http://localhost:8080/k2/ctapi/";

#[derive(Serialize)]
struct Empty();

pub fn post<T>(path: String, payload: &T) -> Result<Response, Error>
    where T: Serialize
{
    let mut url = var("K2_BASE_URL").unwrap_or(BASE_URL.to_string());
    if !url.trim().ends_with("/") {
        url.push_str("/");
    }

    url.push_str(&path);

    let client = Client::new();

    let mut headers = Headers::new();
    headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));

    let body = serde_json::to_string(&payload).unwrap();

    debug!("HTTP POST URL: {}", url);
    debug!("HTTP POST body: {:?}", body);

    let mut builder = client.post(&url);
    builder = builder.headers(headers);
    builder = builder.body(&body[..]);
    builder.send()
}

pub fn simple_post(path: String) -> Result<Response, Error> {
    post(path, &Empty {})
}
