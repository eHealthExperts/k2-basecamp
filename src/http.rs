use super::settings::Settings;
use futures::{self, Future, Stream};
use hyper::{Client, Method, Request, Uri};
use hyper::header::{ContentLength, ContentType};
use std::io::{Error, ErrorKind};
use std::str::{self, FromStr};
use tokio_core::reactor::Core;

pub struct Response {
    pub status: u16,
    pub body: String,
}

pub fn request(path: &str, request_body: Option<String>) -> Result<Response, Error> {
    let mut request = Request::new(Method::Post, try!(uri(path)));
    match request_body {
        Some(json) => {
            debug!("Request body: {}", json);
            request.headers_mut().set(ContentType::json());
            request.headers_mut().set(ContentLength(json.len() as u64));
            request.set_body(json);
        }
        _ => {
            debug!("Empty request body...");
        }
    }

    let mut status: u16 = 0;
    let mut body = String::new();
    {
        let mut core = Core::new().unwrap();
        let client = Client::new(&core.handle());
        let work = client
            .request(request)
            .and_then(|res| {
                status = res.status().clone().into();
                res.body().for_each(|chunk| {
                    body.push_str(str::from_utf8(&*chunk).unwrap());
                    futures::future::ok(())
                })
            })
            .map_err(|err| Error::new(ErrorKind::Other, err));
        try!(core.run(work));
    }

    debug!("Response status: {}", status);
    debug!("Response body: {}", body);
    Ok(Response { status, body })
}

fn uri(path: &str) -> Result<Uri, Error> {
    let mut addr = Settings::base_url();
    addr.push_str(path);
    debug!("Request URL: {}", addr);

    Uri::from_str(&addr).map_err(|err| Error::new(ErrorKind::Other, err))
}

#[cfg(test)]
mod tests {

    use super::{request, Response};
    use rand;
    use std::env;
    use test_server::{self, http};

    #[test]
    fn request_with_body_is_content_type_json() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::BAD_REQUEST);
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let mut body = String::new();
        for _ in 0..10 {
            body.push(rand::random::<u8>() as char);
        }

        let _r = request("", Some(body));

        let (parts, _body) = server.request().unwrap().into_parts();
        assert_eq!(
            parts.headers.get("content-type").unwrap(),
            "application/json"
        );
    }

    #[test]
    fn send_request_body_if_given() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::BAD_REQUEST);
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let mut body = String::new();
        for _ in 0..10 {
            body.push(rand::random::<u8>() as char);
        }

        let _r = request("", Some(body.clone()));

        let (_parts, body) = server.request().unwrap().into_parts();
        assert_eq!(body, body);
    }

    #[test]
    fn if_no_json_is_given_send_empty_request_body() {
        let server = test_server::serve(None);
        server.reply().status(http::StatusCode::BAD_REQUEST);
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let _r = request("", None);

        let (_parts, body) = server.request().unwrap().into_parts();
        assert_eq!(body, String::from(""));
    }

    #[test]
    fn response_contains_status_and_body() {
        let server = test_server::serve(None);
        server
            .reply()
            .status(http::StatusCode::INTERNAL_SERVER_ERROR)
            .body("hello world");
        env::set_var("K2_BASE_URL", format!("http://{}", &server.addr()));

        let response: Response = request("", None).unwrap();
        assert_eq!(response.status, 500);
        assert_eq!(response.body, "hello world");
    }
}
