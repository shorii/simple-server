use router::{RouteKey, Router};
use server::server::Server;
use std::thread;
use std::time;
use types::{HttpMethod, HttpRequest, HttpResponse};

fn foo(_: HttpRequest) -> HttpResponse {
    thread::sleep(time::Duration::from_secs(30));
    HttpResponse {
        version: String::from("1.1"),
        status_code: String::from("200"),
        status_statement: String::from("OK"),
        headers: String::from("Content-Type: application/html"),
        data: String::from("<html><body>foo</body></html>"),
    }
}

fn bar(_: HttpRequest) -> HttpResponse {
    HttpResponse {
        version: String::from("1.1"),
        status_code: String::from("200"),
        status_statement: String::from("OK"),
        headers: String::from("Content-Type: application/html"),
        data: String::from("<html><body>bar</body></html>"),
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut router = Router::new();
    router.add_api(
        RouteKey {
            path: String::from("/api/foo"),
            method: HttpMethod::GET,
        },
        Box::new(foo),
    );
    router.add_api(
        RouteKey {
            path: String::from("/api/bar"),
            method: HttpMethod::GET,
        },
        Box::new(bar),
    );

    let mut server = Server::new("localhost:8080", router);
    server.serve_forever()?;
    Ok(())
}
