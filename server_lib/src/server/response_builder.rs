pub struct ResponseBuilder {
    response_code: String,
    cors_policy: String,
    content_type: String,
    content: String,

    other_headers: Vec<(String, String)>,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        ResponseBuilder {
            response_code: "".to_string(),
            cors_policy: "".to_string(),
            content_type: "".to_string(),
            content: "".to_string(),
            other_headers: vec![],
        }
    }

    pub fn set_response_code<S>(&mut self, code: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.response_code = format!("HTTP/1.1 {}\r\n", code.into());
        self
    }

    pub fn set_content<S>(&mut self, content: S) -> &mut Self
    where
        S: Into<String>,
    {
        let content = content.into();
        if content.len() == 0 {
            return self;
        }

        self.content = format!("Content-Length: {}\r\n\r\n{}", content.len(), content);
        self
    }

    pub fn set_cors_polisy<S>(&mut self, policy: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.cors_policy = format!("Access-Control-Allow-Origin: {}\r\n", policy.into());
        self
    }

    pub fn set_content_type<S>(&mut self, content_type: S) -> &mut Self
    where
        S: Into<String>,
    {
        self.content_type = format!("Content-Type: {}; charset=UTF-8\r\n", content_type.into());
        self
    }

    pub fn add_other_header<S>(&mut self, key_value: (S, S)) -> &mut Self
    where
        S: Into<String>,
    {
        self.other_headers
            .push((key_value.0.into(), key_value.1.into()));
        self
    }

    pub fn build(&self) -> String {
        let mut other_headers = Vec::new();

        for (key, value) in &self.other_headers {
            other_headers.push(format!("{}:{}\r\n", key, value));
        }

        let other_headers = other_headers.concat();

        let response = format!(
            "{}{}{}{}{}",
            self.response_code, self.cors_policy, other_headers, self.content_type, self.content,
        );

        response
    }
}

// our response
// HTTP/1.1 200 OK
// Date: Mon, 01 Dec 2008 00:23:53 GMT
// Server: Apache/2
// Access-Control-Allow-Origin: *
// Keep-Alive: timeout=2, max=100
// Connection: Keep-Alive
// Transfer-Encoding: chunked
// Content-Type: application/json

// request from browser
// GET /resources/public-data/ HTTP/1.1
// Host: bar.other
// User-Agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10.14; rv:71.0) Gecko/20100101 Firefox/71.0
// Accept: text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8
// Accept-Language: en-us,en;q=0.5
// Accept-Encoding: gzip,deflate
// Connection: keep-alive
// Origin: https://foo.example
