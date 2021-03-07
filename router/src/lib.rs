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

struct InnerRoute {
    routes: HashMap<RouteKey, Api>,
}

impl InnerRoute {
    fn new() -> InnerRoute {
        let routes: HashMap<RouteKey, Api> = HashMap::new();
        InnerRoute { routes }
    }

    fn add_api(&mut self, key: RouteKey, api: Api) {
        self.routes.insert(key, api);
    }

    fn get_routes(&self) -> &HashMap<RouteKey, Api> {
        &self.routes
    }
}

lazy_static! {
    static ref ROUTE: Mutex<InnerRoute> = Mutex::new(InnerRoute::new());
}

pub struct Route {}

impl Route {
    pub fn add_api(key: RouteKey, api: Api) {
        ROUTE.lock().unwrap().add_api(key, api);
    }
}

pub struct Router {}

impl Router {
    pub fn dispatch(request: HttpRequest) -> HttpResponse {
        let route_key = RouteKey {
            path: request.path.clone(),
            method: request.method.clone(),
        };
        let inner_route = ROUTE.lock().unwrap();
        let func = inner_route.get_routes().get(&route_key);
        match func {
            Some(f) => f(request),
            None => HttpResponse {
                version: String::from("1.1"),
                status: String::from("404"),
                headers: String::from(""),
                data: String::from(""),
            },
        }
    }
}
