use iron::prelude::*;
use iron::status;
use router::Router;

pub fn get_routes() -> Router {
    let mut router = Router::new();
    router.get("/", index);
    router.get("/:query", handler);
    return router;
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn index(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "")))
}
