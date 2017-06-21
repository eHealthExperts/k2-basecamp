extern crate futures;
extern crate hyper;
extern crate log;
extern crate serde;
extern crate tokio_core;

use super::config;
use futures::Stream;
use hyper::{Client, Method, Request as HyperRequest, Response as HyperResponse, Uri};
use hyper::header::{ContentLength, ContentType};
use tokio_core::reactor::Core;

pub fn request() -> Request {
    let core = Core::new().unwrap();
    let client = Client::new(&core.handle());

    Request {
        addr: config::base_url().clone(),
        core,
        client,
        request: None,
    }
}

pub struct Request {
    addr: String,
    core: Core,
    client: Client<self::hyper::client::HttpConnector>,
    request: Option<HyperRequest>,
}

impl Request {
    pub fn post(mut self, path: &str, body: Option<String>) -> Request {
        let mut request = HyperRequest::new(Method::Post, self.uri(path));
        match body {
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

        self.request = Some(request);
        self
    }

    pub fn response(self) -> Response {
        let req = self.request.unwrap();
        let mut core = self.core;
        let client = self.client;

        let res = core.run(client.request(req)).unwrap();
        Response {
            core: core,
            response: res,
        }
    }

    fn uri(&self, path: &str) -> Uri {
        let uri = format!("{}{}", self.addr, path).parse().unwrap();
        debug!("Request URL: {}", uri);
        uri
    }
}

pub struct Response {
    core: Core,
    response: HyperResponse,
}

impl Response {
    pub fn status(&self) -> u16 {
        debug!("Response status: {}", self.response.status());
        self.response.status().clone().into()
    }

    pub fn body(self) -> String {
        let mut core = self.core;
        let body = core.run(self.response.body().fold(Vec::new(), |mut body, chunk| {
            body.extend_from_slice(&chunk);
            Ok::<_, self::hyper::Error>(body)
        })).unwrap();
        
        let json = String::from_utf8(body).unwrap();
        debug!("Response body: {}", json);
        json
    }
}