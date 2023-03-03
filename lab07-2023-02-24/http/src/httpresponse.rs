use std::collections::HashMap;
use crate::httprequest::HttpRequest;
#[derive(Debug)]
pub enum StatusCode {
    Ok,
    BadRequest,
    NotFound,
}

impl StatusCode {
    fn to_str(&self) -> &'static str {
        match self {
            StatusCode::Ok => "200 OK",
            StatusCode::BadRequest => "400 Bad Request",
            StatusCode::NotFound => "404 Not Found",
        }
    }
}

#[derive(Debug)]
pub struct HttpResponse {
    pub status: StatusCode,
    pub headers: HashMap<String, String>,
    pub msg_body: String,
}

impl HttpResponse {
    fn to_string(&self) -> String {
        let mut res = format!("HTTP/1.1 {}\r\n", self.status.to_str());
        for (key, value) in &self.headers {
            res += &format!("{}: {}\r\n", key, value);
        }
        res += "\r\n";
        res += &self.msg_body;
        res
    }
}

fn handle_request(req: HttpRequest) -> HttpResponse {
    // TODO: Implement the logic for generating an HTTP response based on the request
    let status = StatusCode::Ok;
    let mut headers = HashMap::new();
    headers.insert("Content-Type".into(), "text/plain".into());
    let msg_body = "Hello, world!".into();
    HttpResponse {
        status,
        headers,
        msg_body,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handle_request() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost: localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");
        let req: HttpRequest = s.into();
        let expected_res = HttpResponse {
            status: StatusCode::Ok,
            headers: [("Content-Type".into(), "text/plain".into())].iter().cloned().collect(),
            msg_body: "Hello, world!".into(),
        };
        assert_eq!(handle_request(req).to_string(), expected_res.to_string());
    }
}
