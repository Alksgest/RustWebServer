use crate::server::uri_parser::ParsedUri;

pub trait ControllerBase {
    fn get(&self, opt: &Option<ParsedUri>) -> String {
        "".to_string()
    }
    fn post(&self, opt: &Option<ParsedUri>) -> String {
        "".to_string()
    }
    fn put(&self, opt: &Option<ParsedUri>) -> String {
        "".to_string()
    }
    fn update(&self, opt: &Option<ParsedUri>) -> String {
        "".to_string()
    }
    fn delete(&self, opt: &Option<ParsedUri>) -> String {
        "".to_string()
    }
    fn rout(&self) -> String;
}
