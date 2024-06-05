use crate::{
    http::{request::HttpRequest, response::HttpResponse, HttpStatus},
    utils::split_url_into_parts,
};
use anyhow::Result;

pub(crate) fn root(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::new())
}

pub(crate) fn echo(req: HttpRequest) -> Result<HttpResponse> {
    let parts = split_url_into_parts(req.url);

    // NOTE: This has to be valid as this needs to pass a Regex to get here
    let arg = parts.last().unwrap();
    let response = HttpResponse::new()
        .headers(&[
            ("Content-Type", "text/plain"),
            ("Content-Length", &format!("{}", arg.len())),
        ])
        .body(arg.as_bytes());

    Ok(response)
}

pub(crate) fn user_agent(req: HttpRequest) -> Result<HttpResponse> {
    if let Some(user_agent) = req.headers.get("User-Agent") {
        let response = HttpResponse::new()
            .headers(&[
                ("Content-Type", "text/plain"),
                ("Content-Length", &format!("{}", user_agent.len())),
            ])
            .body(user_agent.as_bytes());

        return Ok(response);
    }

    let bad_req = HttpResponse::new().status(HttpStatus::BadRequest);
    Ok(bad_req)
}
