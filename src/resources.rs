use beermate::Mat;
use deque::Worker;
use iron::prelude::*;
use iron::status;
use iron::modifiers;
use iron::headers;
use router::Router;
use rustc_serialize::json;
use std::sync::Mutex;

pub fn get_routes(worker: Worker<Mat>) -> Router {
    let mut router = Router::new();
    let shared_worker: Mutex<Worker<Mat>> = Mutex::new(worker);
    router.get(
        "/",
        move |request: &mut Request| {
            let worker = shared_worker.lock().unwrap();
            worker.push(Mat{id: 1, level: 0.73, beer_on_mat: true});
            index(request)
        }
    );
    router.get("/:query", handler);
    return router;
}

// Automatically generate `RustcDecodable` and `RustcEncodable` trait
// implementations
#[derive(RustcDecodable, RustcEncodable)]
struct Res {
    code: u8,
    status: String,
}

fn handler(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
    Ok(Response::with((status::Ok, *query)))
}

fn index(_: &mut Request) -> IronResult<Response> {
    Ok(
        Response::with(
            (status::Ok,
            modifiers::Header(headers::ContentType::json()),
            json::encode(&Res {
                code: 200,
                status: "OK".to_string(),
            }).unwrap())
        )
    )
}
