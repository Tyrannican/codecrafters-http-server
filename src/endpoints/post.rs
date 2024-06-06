use crate::{
    http::{request::HttpRequest, response::HttpResponse, HttpStatus},
    utils::split_url_into_parts,
};
use anyhow::Result;

pub(crate) fn files(req: HttpRequest) -> Result<HttpResponse> {
    let parts = split_url_into_parts(req.url);
    let filename = parts.last().unwrap();
    let body = req.body;

    match &req.ctx.workdir {
        Some(wd) => {
            let filename = wd.join(filename);
            std::fs::write(filename, body)?;
            return Ok(HttpResponse::empty().status(HttpStatus::Created));
        }
        None => Ok(HttpResponse::empty().status(HttpStatus::InternalServerError)),
    }
}
