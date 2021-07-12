use crate::server::uri_parser::ParsedUri;

pub trait ControllerBase {
    fn get(&self, opt: &Option<ParsedUri>) -> String {
        format!("{:?}", opt)
    }
    fn post(&self, opt: &Option<ParsedUri>) -> String {
        format!("{:?}", opt)
    }
    fn put(&self, opt: &Option<ParsedUri>) -> String {
        format!("{:?}", opt)
    }
    fn update(&self, opt: &Option<ParsedUri>) -> String {
        format!("{:?}", opt)
    }
    fn delete(&self, opt: &Option<ParsedUri>) -> String {
        format!("{:?}", opt)
    }
    fn route(&self) -> String;
}
