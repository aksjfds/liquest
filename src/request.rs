#![allow(unused)]
#![allow(non_snake_case)]

use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

pub enum Method {
    Get,
    Post,
}

pub struct HttpRequest<'a> {
    Method: Method,
    Domain: &'a str,
    Request: Vec<&'a str>,
}

impl<'a> HttpRequest<'a> {
    pub fn build(url: &'a str, Method: Method) -> Self {
        // url = http://www.baidu.com/index.html
        let mut Request = Vec::new();

        let www = url.find("www").unwrap();
        let com = url.find("com").unwrap();
        let Domain = &url[www..com + 3];

        let uri = url.rfind("/").unwrap();
        let uri = &url[uri..];
        let RequestLine = match Method {
            Method::Get => format!("GET {} HTTP/1.1", uri),
            Method::Post => format!("POST {} HTTP/1.1", uri),
        };

        let RequestLine = Self::as_str(RequestLine);

        Request.push(RequestLine);

        HttpRequest {
            Method,
            Domain,
            Request,
        }
    }

    pub fn header(mut self, key: &str, value: &str) -> Self {
        let header = Self::as_str(format!("{key}:{value}"));
        self.Request.push(header);
        self
    }

    pub fn as_str(string: String) -> &'a str {
        Box::leak(Box::new(string))
    }

    pub fn to_string(&mut self) -> String {
        self.Request.push("\r\n");
        self.Request.join("\r\n")
    }

    pub fn connect(&mut self) -> String {
        let address = (self.Domain, 80);
        let mut stream = TcpStream::connect(address).unwrap();

        stream.write_all(self.to_string().as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        response
    }
}

#[test]
fn test() {
    let mut http_request = HttpRequest::build("https://www.baidu.com/index.html", Method::Get)
        .header("Accept", "text/html,application/xhtml+xml,application/json")
        .header("Connection", "close");

    let response = http_request.connect();
    println!("{}", response);
}
