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

pub(crate) fn files(req: HttpRequest) -> Result<HttpResponse> {
    let parts = split_url_into_parts(req.url);
    // NOTE: always valid as it passes Regex to get here
    let filename = parts.last().unwrap();

    match &req.ctx.workdir {
        Some(wd) => {
            let fp = wd.join(filename);
            if !fp.exists() {
                return Ok(HttpResponse::new().status(HttpStatus::NotFound));
            }
            let buf = std::fs::read(fp)?;
            let response = HttpResponse::new()
                .headers(&[
                    ("Content-Type", "application/octet-stream"),
                    ("Content-Length", &format!("{}", buf.len())),
                ])
                .body(&buf);

            Ok(response)
        }
        None => Ok(HttpResponse::new().status(HttpStatus::InternalServerError)),
    }
}
