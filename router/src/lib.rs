use std::collections::HashMap;
use types::{HttpMethod, HttpRequest, HttpResponse};

pub type Api = Box<dyn (Fn(HttpRequest) -> HttpResponse) + Send + Sync>;

#[derive(Hash, PartialEq, Eq)]
pub struct RouteKey {
    pub path: String,
    pub method: HttpMethod,
}

pub struct Router {
    route_map: HashMap<RouteKey, Api>,
}

impl Router {
    pub fn new() -> Self {
        let route_map: HashMap<RouteKey, Api> = HashMap::new();
        Router { route_map }
    }

    pub fn add_api(&mut self, key: RouteKey, api: Api) {
        self.route_map.insert(key, api);
    }

    pub fn dispatch(&self, request: HttpRequest) -> HttpResponse {
        let route_key = RouteKey {
            path: request.path.clone(),
            method: request.method.clone(),
        };
        let func = self.route_map.get(&route_key);
        match func {
            Some(f) => f(request),
            None => HttpResponse {
                version: String::from("1.1"),
                status_code: String::from("404"),
                status_statement: String::from("Not Found"),
                headers: String::from(""),
                data: String::from(""),
            },
        }
    }
}
