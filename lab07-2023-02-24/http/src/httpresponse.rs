// Define an enumeration to represent HTTP status codes
#[derive(Debug, PartialEq)]
pub enum StatusCode {
    OK,
    NotFound,
    Uninitialized,
}

impl StatusCode {
    // Implement a method to convert a StatusCode to a string representation
    fn to_string(&self) -> String {
        match self {
            StatusCode::OK => "200 OK".to_string(),
            StatusCode::NotFound => "404 Not Found".to_string(),
            StatusCode::Uninitialized => "".to_string(),
        }
    }
}

// Define a struct to represent an HTTP response
#[derive(Debug, PartialEq)]
pub struct HttpResponse {
    pub status_code: StatusCode,
    pub headers: Vec<(String, String)>,
    pub msg_body: String,
}

impl HttpResponse {
    // Implement a constructor for HttpResponse that takes a StatusCode and a message body
    pub fn new(status_code: StatusCode, msg_body: String) -> HttpResponse {
        HttpResponse {
            status_code: status_code,
            headers: vec![("Content-Length".to_string(), msg_body.len().to_string())],
            msg_body: msg_body,
        }
    }

    // Implement a method to convert an HttpResponse to a string representation
    pub fn to_string(&self) -> String {
        let mut res = format!("HTTP/1.1 {}\r\n", self.status_code.to_string());
        for (key, value) in &self.headers {
            res.push_str(&format!("{}: {}\r\n", key, value));
        }
        res.push_str("\r\n");
        res.push_str(&self.msg_body);
        res
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_code_to_string() {
        assert_eq!(StatusCode::OK.to_string(), "200 OK");
        assert_eq!(StatusCode::NotFound.to_string(), "404 Not Found");
        assert_eq!(StatusCode::Uninitialized.to_string(), "");
    }

    #[test]
    fn test_http_response_new() {
        let response = HttpResponse::new(StatusCode::OK, "Hello, world!".to_string());
        assert_eq!(response.status_code, StatusCode::OK);
        assert_eq!(response.headers, vec![("Content-Length".to_string(), "13".to_string())]);
        assert_eq!(response.msg_body, "Hello, world!".to_string());
    }

    #[test]
    fn test_http_response_to_string() {
        let response = HttpResponse::new(StatusCode::NotFound, "Not found".to_string());
        let expected = "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot found".to_string();
        assert_eq!(response.to_string(), expected);
    }
}
