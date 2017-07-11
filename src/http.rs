extern crate futures;
extern crate hyper;
extern crate log;
extern crate serde;
extern crate tokio_core;

use super::config;
use futures::{Future, Stream};
use hyper::{Client, Method, Request, Uri};
use hyper::header::{ContentLength, ContentType};
use std::io;
use std::str;
use tokio_core::reactor::Core;

pub struct Response {
    pub status: u16,
    pub body: String,
}

pub fn request(path: &str, request_body: Option<String>) -> Result<Response, io::Error> {
    let mut request = Request::new(Method::Post, uri(path));
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
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err));
        try!(core.run(work));
    }

    debug!("Response status: {}", status);
    debug!("Response body: {}", body);
    Ok(Response { status, body })
}

fn uri(path: &str) -> Uri {
    let addr = config::base_url().clone();
    let uri = format!("{}{}", addr, path).parse().unwrap();
    debug!("Request URL: {}", uri);
    uri
}
