extern crate futures;
extern crate hyper;
extern crate log;
extern crate serde;
extern crate tokio_core;

use super::config;
use futures::{Future, Stream};
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
    let mut addr = config::base_url().clone();
    addr.push_str(path);
    debug!("Request URL: {}", addr);

    Uri::from_str(&addr).map_err(|err| Error::new(ErrorKind::Other, err))
}

#[cfg(test)]
mod tests {

    use super::{request, Response};
    use antidote::Mutex;
    use rand;
    use std::io::Read;

    #[test]
    fn request_with_body_is_content_type_json() {
        let mut body = String::new();
        for _ in 0..10 {
            body.push(rand::random::<u8>() as char);
        }

        let shutdown = test_server!((request: &Request) {
            let header = request.header("Content-Type");
            assert!(header.unwrap().starts_with("application/json"));

            ::rouille::Response::empty_404()
        });

        let _r = request("", Some(body));

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn send_request_body_if_given() {
        let mut body = String::new();
        for _ in 0..10 {
            body.push(rand::random::<u8>() as char);
        }

        let holder = Mutex::new(hashmap!["body" => body.clone()]);

        let shutdown = test_server!((request: &Request) {
            let mut body = String::new();
            let data = request.data();
            let _ = data.unwrap().read_to_string(&mut body);

            assert_eq!(&body, holder.lock().get("body").unwrap());

            ::rouille::Response::empty_404()
        });

        let _r = request("", Some(body));

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn if_no_json_is_given_send_empty_request_body() {
        let shutdown = test_server!((request: &Request) {
            let mut body = String::new();
            let data = request.data();
            let _ = data.unwrap().read_to_string(&mut body);

            assert_eq!(&body, "");

            ::rouille::Response::empty_404()
        });

        let _r = request("", None);

        // kill server thread
        let _ = shutdown.send(());
    }

    #[test]
    fn response_contains_status_and_body() {
        let shutdown = test_server!((request: &Request) {
            ::rouille::Response::text("hello world").with_status_code(500)
        });

        let response: Response = request("", None).unwrap();
        assert_eq!(response.status, 500);
        assert_eq!(response.body, "hello world");

        // kill server thread
        let _ = shutdown.send(());
    }
}
