#[derive(Clone)]
pub struct ApiSettings {
    pub adress: String,
    pub port: String,
    pub buffer_size: usize,
}

impl ApiSettings {
    pub fn new(adress: &str, port: &str, buffer_size: usize) -> Self {
        Self {
            adress: adress.to_string(),
            port: port.to_string(),
            buffer_size,
        }
    }

    pub fn create_url(&self) -> String {
        format!("{}:{}", self.adress, self.port)
    }
}
