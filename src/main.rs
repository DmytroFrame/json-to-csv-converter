use iron::Iron;
use mount::Mount;
use router::Router;
use staticfile::Static;
use std::path::Path;

mod routers;
use routers::convert::convert;
use routers::handler::handler;

fn main() {
    let mut router = Router::new();
    router.get("/", handler, "handler");
    router.post("/convert", convert, "convert");

    let mut mount = Mount::new();
    mount.mount("/api/", router);
    mount.mount("/", Static::new(Path::new("client")));

    Iron::new(mount)
        .http("localhost:3000")
        .expect("Server is broke");
}
