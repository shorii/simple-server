#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;
use std::sync::Mutex;
use types::{HttpMethod, HttpRequest, HttpResponse};

pub type Api = Box<dyn (Fn(HttpRequest) -> HttpResponse) + Send + Sync>;

#[derive(Hash, PartialEq, Eq)]
pub struct RouteKey {
    path: String,
    method: HttpMethod,
}

lazy_static! {
    static ref ROUTE: Mutex<HashMap<RouteKey, Api>> = Mutex::new(HashMap::new());
}

pub struct Route {}

impl Route {
    pub fn add_api(key: RouteKey, api: Api) {
        let mut route_map = ROUTE.lock().unwrap();
        route_map.insert(key, api);
    }
}

pub struct Router {}

impl Router {
    pub fn dispatch(request: HttpRequest) -> HttpResponse {
        let route_key = RouteKey {
            path: request.path.clone(),
            method: request.method.clone(),
        };
        let route_map = ROUTE.lock().unwrap();
        let func = route_map.get(&route_key);
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
