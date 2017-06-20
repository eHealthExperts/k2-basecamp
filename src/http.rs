extern crate hyper;
extern crate log;
extern crate serde;
extern crate serde_json;

use super::config;
use hyper::{Client, Error};
use hyper::client::response::Response;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use hyper::status::StatusCode;
use serde::Serialize;
use std::io::Read;

pub enum HttpStatus {
    Ok,
    Other,
}

#[derive(Serialize)]
struct Empty();

pub fn simple_post(path: String) -> Result<Response, Error> {
    post(path, &Empty {})
}

pub fn post<T>(path: String, payload: &T) -> Result<Response, Error>
where
    T: Serialize,
{
    let url = format!("{}{}", config::base_url(), path);
    debug!("Request URL: {}", url);

    let body = serde_json::to_string(&payload).unwrap();
    debug!("Request body: {}", body);

    let mut headers = Headers::new();
    headers.set(ContentType(
        Mime(TopLevel::Application, SubLevel::Json, vec![]),
    ));

    let client = Client::new();
    let mut builder = client.post(&url);
    builder = builder.headers(headers);
    builder = builder.body(&body[..]);
    builder.send()
}

pub fn extract_response(mut response: Response) -> (HttpStatus, String) {
    debug!("Response status: {}", response.status);
    let status = match response.status {
        StatusCode::Ok => HttpStatus::Ok,
        _ => HttpStatus::Other,
    };

    let mut body = String::new();
    response.read_to_string(&mut body).unwrap();;
    debug!("Response body: {}", body);

    (status, body)
}