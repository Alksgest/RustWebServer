use std::collections::HashMap;
use std::io::prelude::*;

#[derive(Debug)]
pub struct ParsedRequest {
    rest_method: String,
    route: String,
    query_params: HashMap<String, Option<String>>,
    protocol: String,
    content: Option<String>,
}

impl ParsedRequest {
    pub fn new(
        rest_method: String,
        route: String,
        query_params: HashMap<String, Option<String>>,
        protocol: String,
        content: Option<String>,
    ) -> Self {
        ParsedRequest {
            rest_method,
            route,
            query_params,
            protocol,
            content,
        }
    }

    pub fn route(&self) -> String {
        self.route.clone()
    }

    pub fn rest_method(&self) -> String {
        self.rest_method.clone()
    }

    pub fn content(&self) -> String {
        match &self.content {
            Some(val) => val.clone(),
            None => String::from("")
        }
    }

    pub fn query_params_len(&self) -> usize {
        self.query_params.len()
    }

    pub fn query_params(&self) -> HashMap<String, Option<String>> {
        self.query_params.clone()
    }
}

impl Clone for ParsedRequest {
    fn clone(&self) -> Self {
        ParsedRequest {
            rest_method: self.rest_method.clone(),
            route: self.route.clone(),
            query_params: self.query_params.clone(),
            protocol: self.protocol.clone(),
            content: self.content.clone(),
        }
    }
}

// For now we cannot read content of request
pub struct RequestParser {}

// GET /test?value=sus&other_value=kus%dus HTTP/1.1
impl RequestParser {
    pub fn parse(buffer: &Vec<u8>) -> ParsedRequest {
        let headers: Vec<_> = buffer
            .lines()
            .filter_map(|el| match el {
                Ok(val) => Some(val.clone()),
                Err(_) => None,
            })
            .collect();
        let first_header = headers.get(0);

        let header = match first_header {
            Some(val) => Some(val),
            None => None,
        }
        .unwrap()
        .to_string();

        let (rest_method, route, query_params, protocol) = RequestParser::parse_header(header);
        let body = RequestParser::parse_content(&headers);

        ParsedRequest {
            rest_method: rest_method,
            route: route,
            query_params: query_params,
            protocol: protocol,
            content: body,
        }
    }

    fn parse_content(lines: &Vec<std::string::String>) -> Option<String> {
        let content_position = lines.iter().position(|el| el.is_empty()).unwrap();
        let content: Vec<_> = lines
            .iter()
            .skip(content_position)
            .map(|el| el.to_string())
            .collect();
        let content = content.join("");

        Some(content)
    }

    fn parse_header(header: String) -> (String, String, HashMap<String, Option<String>>, String) {
        let first_split: Vec<&str> = header.split(" ").collect();
        let rest_method = first_split[0];
        let (route, query_params) = RequestParser::parse_uri(first_split[1]);
        let protocol = first_split[2];
        (
            rest_method.to_string(),
            route,
            query_params,
            protocol.to_string(),
        )
    }

    fn parse_uri(uri: &str) -> (String, HashMap<String, Option<String>>) {
        let second_split: Vec<&str> = uri.split("?").collect();
        let route = second_split[0];
        let mut query_params = HashMap::new();
        if second_split.len() > 1 {
            let third_split = second_split[1].split("&");
            for val in third_split {
                let split: Vec<&str> = val.split("=").collect();
                let key = split[0].to_string();
                let value = match split.get(1) {
                    Some(val) => Option::from(val.to_string()),
                    _ => None,
                };
                query_params.insert(key, value);
            }
        }
        (route.to_string(), query_params)
    }
}
