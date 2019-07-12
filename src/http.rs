use std::fmt;
use std::error::Error;
use std::str;
use std::io::Read;
use std::time::Duration;
use std::collections::BTreeMap;

extern crate curl;
use self::curl::easy::{Easy, List};

extern crate percent_encoding;

#[derive(PartialEq, Debug)]
pub enum HttpMethod {
    #[allow(dead_code)]
    GET,
    POST,
    PUT,
}

pub type HttpErrorString = String;
pub struct HttpResponse {
    pub code: Option<u32>,
    pub data: Result<String, HttpErrorString>,
}

impl HttpResponse {
    #[allow(dead_code)]
    pub fn unwrap(self) -> String { self.data.unwrap() }
    #[allow(dead_code)]
    pub fn print(&self) {
        let code: i32 = match self.code {
            Some(x) => { x as i32 }
            None => -1
        };
        println!("Code: {}", code);
        match self.data {
            Ok(ref s) => {println!("{}", s)}
            Err(ref s) => {println!("ERROR: {}", s)}
        }
    }
}

impl fmt::Display for HttpResponse {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let code: i32 = match self.code {
            Some(x) => { x as i32 }
            None => -1
        };
        let _ = write!(f, "Code: {}\n", code);
        match self.data {
            Ok(ref s) => {write!(f, "Response: {}", s)}
            Err(ref s) => {write!(f, "ERROR: {}", s)}
        }
    }
}

pub struct QueryString {
    map: BTreeMap<String,String>,
}
impl QueryString {
    pub fn new() -> QueryString { QueryString { map: BTreeMap::<String,String>::new() } }
    #[allow(dead_code)]
    pub fn add_opt(&mut self, key: &str, value: Option<String>) -> &mut QueryString {
        match value {
            Some(v) => { self.map.insert(key.to_string(), v); },
            None => {},
        }
        self
    }
    pub fn add<A>(&mut self, key: &str, value: A) -> &mut QueryString
        where A: ToString {
        self.map.insert(key.to_string(), value.to_string());
        self
    }
    pub fn build(&self) -> String {
        let mut s = String::new();
        for (key, val) in &self.map {
            match s.len() {
                0 => { } // '?' inserted in HTTP layer
                _ => { s = s + "&"; }
            }
            s = s + &format!("{}={}", key, val);
        }
        s
    }
}

pub fn http(url: &str, query: Option<&str>, body: Option<&str>,
            method: HttpMethod,) -> HttpResponse {
    let mut headers = List::new();
    println!("HTTP URL: {:?} {}\nQuery: {:?}\nBody: {:?}", method, url, query, body);
    let data = match method {
        HttpMethod::POST => {
            match query {
                Some(q) => {
                    let enc_query = percent_encoding::utf8_percent_encode(&q, percent_encoding::QUERY_ENCODE_SET).collect::<String>();
                    enc_query
                },
                None => {
                    let header = format!("Content-Type: application/json");
                    headers.append(&header).unwrap();
                    body.unwrap_or("").to_string()
                }
            }
        },
        _ => { body.unwrap_or("").to_string() },
    };
    let mut data = data.as_bytes();

    let url = match method {
        HttpMethod::GET | HttpMethod::PUT => match query {
            None => url.to_string(),
            Some(q) => format!("{}?{}", url, q),
        },
        _ => url.to_string()

    };
    let mut response = None;
    let mut json_bytes = Vec::<u8>::new();
    {
        let mut easy = Easy::new();
        let _ = easy.timeout(Duration::new(20,0)); // 20 sec timeout
        easy.url(&url).unwrap();
        match method {
            HttpMethod::POST => {
                easy.post(true).unwrap();
                easy.post_field_size(data.len() as u64).unwrap();
            }
            HttpMethod::PUT => {
                easy.put(true).unwrap();
                easy.post_field_size(data.len() as u64).unwrap();
            }
            _ => {}
        }

        {
            let mut transfer = easy.transfer();
            if method == HttpMethod::POST || method == HttpMethod::PUT {
                transfer.read_function(|buf| {
                    Ok(data.read(buf).unwrap_or(0))
                }).unwrap();
            }
            transfer.write_function(|x| {
                json_bytes.extend(x);
                Ok(x.len())
            }).unwrap();
            match transfer.perform() {
                Err(x) => {
                    let result: Result<String,String> = Err(x.description().to_string());
                    return HttpResponse {code: response, data: result }
                }
                _ => {}
            };
        }
        response = match easy.response_code() {
            Ok(code) => { Some(code) }
            _ => { None }
        };
    }
    let result: Result<String,String> = match String::from_utf8(json_bytes) {
        Ok(x) => { Ok(x) }
        Err(x) => { Err(x.utf8_error().description().to_string()) }
    };
    println!("HTTP response: {}", result.clone().unwrap());
    HttpResponse {code: response, data: result }
}
