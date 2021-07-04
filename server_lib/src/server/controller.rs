use crate::server::uri_parser::ParsedUri;

#[allow(dead_code)]
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
    fn rout(&self) -> String;
}
