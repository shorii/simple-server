#[derive(Clone, Debug)]
pub struct HttpResponse {
    pub version: String,
    pub status: String,
    pub headers: String,
    pub data: String,
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum HttpMethod {
    GET,
    POST,
    PATCH,
    DELETE,
    OPTION,
}

#[derive(Clone, Debug)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub path: String,
    pub version: String,
    pub headers: u32,
    pub data: String,
}
