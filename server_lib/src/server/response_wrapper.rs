pub mod response_wrapper {
    use crate::server::response_builder::ResponseBuilder;

    // 200 OK
    // 201 Created
    // 400 Bad Request
    // 401 Unauthorized
    // 403 Forbidden
    // 404 Not Found
    // 500 Internal Server Error

    pub fn success(content: Option<String>) -> String {
        create_response("200 OK", content)
    }

    pub fn created(content: Option<String>) -> String {
        create_response("201 Created", content)
    }

    pub fn bad_request(content: Option<String>) -> String {
        create_response("400 Bad Request", content)
    }

    pub fn unauthorized(content: Option<String>) -> String {
        create_response("401 Unauthorized", content)
    }

    pub fn forbidden(content: Option<String>) -> String {
        create_response("403 Forbidden", content)
    }

    pub fn not_found(content: Option<String>) -> String {
        create_response("404 Not Found", content)
    }

    pub fn method_not_allowed(content: Option<String>) -> String {
        create_response("405 Method Not Allowed", content)
    }

    pub fn internal_server_error(content: Option<String>) -> String {
        create_response("500 Internal Server Error", content)
    }

    fn create_response(status: &str, content: Option<String>) -> String {
        let mut builder = ResponseBuilder::new();
        let content = match content {
            Some(val) => val.into(),
            None => "".to_string(),
        };

        builder
            .set_response_code(status)
            .set_cors_polisy("*")
            .set_content_type("application/json")
            .set_content(content)
            .build()
    }
}
