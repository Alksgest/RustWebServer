use crate::server::request_parser::ParsedRequest;

pub trait ControllerBase {
    fn get(&self, opt: &Option<ParsedRequest>) -> String;
    fn post(&self, opt: &Option<ParsedRequest>) -> String;
    fn put(&self, opt: &Option<ParsedRequest>) -> String;
    fn update(&self, opt: &Option<ParsedRequest>) -> String;
    fn delete(&self, opt: &Option<ParsedRequest>) -> String;
    fn route(&self) -> String;
}
