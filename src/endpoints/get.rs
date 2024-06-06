use crate::{
    http::{request::HttpRequest, response::HttpResponse, HttpStatus},
    utils::split_url_into_parts,
};
use anyhow::Result;

pub(crate) fn root(_req: HttpRequest) -> Result<HttpResponse> {
    Ok(HttpResponse::empty())
}

pub(crate) fn echo(req: HttpRequest) -> Result<HttpResponse> {
    let parts = split_url_into_parts(&req.url);

    // NOTE: This has to be valid as this needs to pass a Regex to get here
    let arg = parts.last().unwrap();
    let response = HttpResponse::new(&req)
        .headers(&[("Content-Type", "text/plain")])
        .body(arg.as_bytes());

    Ok(response)
}

pub(crate) fn user_agent(req: HttpRequest) -> Result<HttpResponse> {
    if let Some(user_agent) = req.get_header("User-Agent") {
        let response = HttpResponse::new(&req)
            .headers(&[("Content-Type", "text/plain")])
            .body(user_agent.as_bytes());

        return Ok(response);
    }

    let bad_req = HttpResponse::empty().status(HttpStatus::BadRequest);
    Ok(bad_req)
}

pub(crate) fn files(req: HttpRequest) -> Result<HttpResponse> {
    let parts = split_url_into_parts(&req.url);
    // NOTE: always valid as it passes Regex to get here
    let filename = parts.last().unwrap();

    match &req.ctx.workdir {
        Some(wd) => {
            let fp = wd.join(filename);
            if !fp.exists() {
                return Ok(HttpResponse::empty().status(HttpStatus::NotFound));
            }
            let buf = std::fs::read(fp)?;
            let response = HttpResponse::new(&req)
                .headers(&[("Content-Type", "application/octet-stream")])
                .body(&buf);

            Ok(response)
        }
        None => Ok(HttpResponse::empty().status(HttpStatus::InternalServerError)),
    }
}
