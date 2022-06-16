use hyper::{Request, Response, Body, StatusCode, Method};



pub async fn process(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/") => {
            let whole_body = hyper::body::to_bytes(req.into_body()).await?;
            let body_vec = whole_body.iter().cloned().collect::<Vec<u8>>();
            
            Ok(Response::new(Body::from("ok")))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}