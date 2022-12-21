use iron::{status, IronResult, Request, Response};

pub fn handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((
        status::Ok,
        r###"{"message": "This API""}"###,
    )))
}
