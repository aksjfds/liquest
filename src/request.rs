#![allow(unused)]
#![allow(non_snake_case)]

use std::{
    collections::HashMap,
    io::{Read, Write},
    net::TcpStream,
};

// use native_tls::TlsConnector;
// use openssl::ssl::SslContext;

pub enum Method {
    Get,
    Post,
}
pub struct RequestHeaders<'a> {
    Headers: HashMap<&'a str, &'a str>,
}

impl<'a> RequestHeaders<'a> {
    fn new(headers: HashMap<&'a str, &'a str>) -> Self {
        RequestHeaders { Headers: headers }
    }

    fn parse(headers: &'a str, f: impl FnOnce(&str) -> HashMap<&str, &str>) -> Self {
        let headers = f(headers);

        RequestHeaders { Headers: headers }
    }

    fn insert(&mut self, key: &'a str, value: &'a str) {
        self.Headers.insert(key, value);
    }

    fn extend(&mut self, headers: HashMap<&'a str, &'a str>) {
        self.Headers.extend(headers);
    }

    fn to_string(&self) -> String {
        let headers: Vec<_> = self
            .Headers
            .iter()
            .map(|(key, value)| format!("{key}:{value}"))
            .collect();
        headers.join("\r\n")
    }
}

pub struct HttpRequest<'a> {
    Method: Method,
    Domain: &'a str,
    RequestLine: String,
    RequestHeaders: RequestHeaders<'a>,
    RequestBody: Option<String>,
    // Https: bool,
}

impl<'a> HttpRequest<'a> {
    pub fn new(url: &'a str, Method: Method) -> Self {
        // url = http://www.baidu.com/index.html

        let www = url.find("www").unwrap();
        let com = url.find("com").unwrap();
        let Domain = &url[www..com + 3];

        let uri = url.rfind("/").unwrap();
        let uri = &url[uri..];
        let RequestLine = match Method {
            Method::Get => format!("GET {} HTTP/1.1\r\n", uri),
            Method::Post => format!("POST {} HTTP/1.1\r\n", uri),
        };

        let mut headers = HashMap::new();
        headers.insert("Host", Domain);
        let RequestHeaders = RequestHeaders::new(headers);

        HttpRequest {
            Method,
            Domain,
            RequestLine,
            RequestHeaders,
            RequestBody: None,
        }
    }

    pub fn insert_header(mut self, key: &'a str, value: &'a str) -> Self {
        self.RequestHeaders.insert(key, value);
        self
    }

    pub fn extend_headers(mut self, headers: HashMap<&'a str, &'a str>) -> Self {
        self.RequestHeaders.extend(headers);
        self
    }

    pub fn to_u8(&self) -> String {
        let mut u8_string = String::new();

        u8_string.push_str(&self.RequestLine);
        u8_string.push_str(&self.RequestHeaders.to_string());
        u8_string.push_str("\r\n");

        if let Method::Get = self.Method {
            u8_string.push_str("\r\n");
        } else {
        }

        u8_string
    }

    pub fn connect(&self) -> String {
        let address = (self.Domain, 80);
        let mut stream = TcpStream::connect(address).unwrap();

        stream.write_all(self.to_u8().as_bytes()).unwrap();

        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();

        response
    }
}

#[test]
fn test() {
    let mut http_request = HttpRequest::new("https://www.baidu.com/index.html", Method::Get)
        .insert_header("Accept", "text/html,application/xhtml+xml,application/json")
        .insert_header("Connection", "close");

    let response = http_request.connect();
    println!("{}", response);
    println!("{:#?}", http_request.to_u8());
}

#[test]
fn test_headers_parse() {
    let headers = "Host: www.bing.com
User-Agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:134.0) Gecko/20100101 Firefox/134.0
Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
Accept-Language: zh-CN,zh;q=0.8,zh-TW;q=0.7,zh-HK;q=0.5,en-US;q=0.3,en;q=0.2
Accept-Encoding: gzip, deflate, br, zstd
Connection: keep-alive
Upgrade-Insecure-Requests: 1
Sec-Fetch-Dest: document
Sec-Fetch-Mode: navigate
Sec-Fetch-Site: same-origin
Sec-Fetch-User: ?1
Priority: u=0, i
Pragma: no-cache
Cache-Control: no-cache";
    let headers = RequestHeaders::parse(headers, |headers| {
        headers
            .lines()
            .map(|header| {
                let mut temp = header.split(": ");
                (temp.next().unwrap(), temp.next().unwrap())
            })
            .collect()
    });

    println!("{:#?}", headers.Headers);
}
