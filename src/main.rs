extern crate iron;
extern crate router;

use iron::prelude::*;
use iron::status;
use router::Router;

fn to_vec(str: &str) -> Vec<Vec<u8>> {
    vec![str.as_bytes().to_vec()]
}

fn index_handler(_: &mut Request) -> IronResult<Response> {
    let mut response = Response::with((status::Ok, "Главная" ));
    response.headers.set_raw("Content-Type",   to_vec("text/html; charset=utf-8"));
    Ok(response)
}

fn admin_handler(_: &mut Request) -> IronResult<Response> {
    let mut response = Response::with((status::Ok, "Админка" ));
    response.headers.set_raw("Content-Type",   to_vec("text/html; charset=utf-8"));
    Ok(response)
}

fn main() {
    let mut router = Router::new();
    router.get("/", index_handler, "index");
    router.get("/admin", admin_handler, "admin");

    let server = Iron::new(router);

    println!("Start Server on port 3000...");
    server.http("localhost:3000").unwrap();
    println!("Started!");

}