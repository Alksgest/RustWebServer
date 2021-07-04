use std::collections::HashMap;

// For now we cannot read content of request
pub struct UriParser {}

#[derive(Debug)]
pub struct ParsedUri {
    rest_method: String,
    route: String,
    query_params: HashMap<String, Option<String>>,
    protocol: String,
}

impl ParsedUri {
    pub fn new(
        rest_method: String,
        route: String,
        query_params: HashMap<String, Option<String>>,
        protocol: String,
    ) -> Self {
        ParsedUri {
            rest_method,
            route,
            query_params,
            protocol,
        }
    }

    pub fn route(&self) -> String {
        self.route.clone()
    }

    pub fn rest_method(&self) -> String {
        self.rest_method.clone()
    }

    pub fn query_params(&self) -> HashMap<String, Option<String>> {
        self.query_params.clone()
    }
}

impl Clone for ParsedUri {
    fn clone(&self) -> Self {
        ParsedUri {
            rest_method: self.rest_method.clone(),
            route: self.route.clone(),
            query_params: self.query_params.clone(),
            protocol: self.protocol.clone(),
        }
    }
}

// GET /test?value=sus&other_value=kus%dus HTTP/1.1
impl UriParser {
    pub fn parse_header(header: String) -> ParsedUri {
        let first_split: Vec<&str> = header.split(" ").collect();
        let rest_method = first_split[0];
        let (route, query_params) = UriParser::parse_uri(first_split[1]);
        let protocol = first_split[2];
        ParsedUri {
            rest_method: rest_method.to_string(),
            route,
            query_params,
            protocol: protocol.to_string(),
        }
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
