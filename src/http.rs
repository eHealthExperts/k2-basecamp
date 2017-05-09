#[cfg(test)]
extern crate http_stub;
extern crate hyper;
extern crate log;
extern crate serde;
extern crate serde_json;

use super::config;
use hyper::Client;
use hyper::Error;
use hyper::client::response::Response;
use hyper::header::{Headers, ContentType};
use hyper::mime::{Mime, TopLevel, SubLevel};
use serde::Serialize;

#[derive(Serialize)]
struct Empty();

pub fn simple_post(path: String) -> Result<Response, Error> {
    post(path, &Empty {})
}

pub fn post<T>(path: String, payload: &T) -> Result<Response, Error>
    where T: Serialize
{
    let mut url = config::base_url();
    url.push_str(&path);
    debug!("HTTP POST URL: {}", url);

    let body = serde_json::to_string(&payload).unwrap();
    debug!("HTTP POST body: {:?}", body);

    let mut headers = Headers::new();
    headers.set(ContentType(Mime(TopLevel::Application, SubLevel::Json, vec![])));

    let client = Client::new();
    let mut builder = client.post(&url);
    builder = builder.headers(headers);
    builder = builder.body(&body[..]);
    builder.send()
}

#[cfg(test)]
mod tests {
    use self::http_stub as hs;
    use self::http_stub::HttpStub;
    use super::*;
    use std::env;
    use std::io::Read;

    const BASE_URL_KEY: &'static str = "K2_BASE_URL";

    #[test]
    fn simple_post_is_returning_error() {
        env::remove_var(BASE_URL_KEY);

        let res = simple_post(String::from("hello"));
        assert!(res.is_err());
    }

    #[test]
    #[ignore]
    fn simple_post_is_sending_empty_body() {
        let url = HttpStub::run(|mut stub| {
                                    stub.got_body("");
                                    stub.got_path("/hello");
                                    stub.got_method(hs::Method::Post);

                                    stub.send_status(hs::StatusCode::Ok);
                                    stub.send_body("world");
                                });

        env::set_var(BASE_URL_KEY, url);

        let mut res = simple_post(String::from("hello")).unwrap();
        assert_eq!(res.status, hyper::Ok);

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        assert_eq!(body, "world");
    }

    #[test]
    fn post_is_returning_error() {
        env::remove_var(BASE_URL_KEY);

        #[derive(Serialize)]
        struct Data {
            name: String,
        };

        let data = Data { name: String::from("hello") };

        let res = post(String::from("hello"), &data);
        assert!(res.is_err());
    }

    #[test]
    #[ignore]
    fn post_is_sending_payload_as_json_in_body() {
        let url = HttpStub::run(|mut stub| {
            stub.got_path("/hello");
            stub.got_method(hs::Method::Post);
            stub.got_header("content-type", "application/json");
            stub.got_body(r#"\{"name":"hello"\}"#);

            stub.send_status(hs::StatusCode::Ok);
            stub.send_body("world");
        });

        env::set_var(BASE_URL_KEY, url);

        #[derive(Serialize)]
        struct Data {
            name: String,
        };

        let data = Data { name: String::from("hello") };

        let mut res = post(String::from("hello"), &data).unwrap();
        assert_eq!(res.status, hyper::Ok);

        let mut body = String::new();
        res.read_to_string(&mut body).unwrap();
        assert_eq!(body, "world");
    }
}
