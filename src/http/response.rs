use anyhow::Result;
use std::{collections::HashMap, io::Write};

use super::{encoder::Encoder, request::HttpRequest, supported_encoding, HttpStatus};

pub(crate) struct HttpResponse {
    http_version: String,
    status: HttpStatus,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

fn prepop_headers(req: &HttpRequest) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    if let Some(content_encoding) = req.get_header("accept-encoding") {
        let content_encoding = content_encoding
            .split(", ")
            .filter(|ce| supported_encoding(ce))
            .collect::<Vec<&str>>();

        if content_encoding.is_empty() {
            return headers;
        }

        let value = content_encoding.join(", ");
        headers.insert("content-encoding".to_string(), value);
    }

    headers
}

impl HttpResponse {
    pub(crate) fn new(req: &HttpRequest) -> Self {
        let headers = prepop_headers(req);

        Self {
            http_version: "HTTP/1.1".to_string(),
            status: HttpStatus::OK,
            headers,
            body: Vec::new(),
        }
    }

    pub(crate) fn empty() -> Self {
        Self {
            http_version: "HTTP/1.1".to_string(),
            status: HttpStatus::OK,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub(crate) fn status(mut self, status: HttpStatus) -> Self {
        self.status = status;
        self
    }

    pub(crate) fn headers(mut self, headers: &[(&str, &str)]) -> Self {
        for header in headers {
            self.headers
                .insert(header.0.to_string(), header.1.to_string());
        }
        self
    }

    pub(crate) fn body(mut self, body: &[u8]) -> Self {
        if let Some(encoding) = self.headers.get("content-encoding") {
            let encoding_values = encoding
                .split(", ")
                .map(|e| e.to_string())
                .collect::<Vec<String>>();
            let chosen = encoding_values.first().unwrap();
            let encoder = Encoder::new(chosen);
            self.body = encoder.encode(body);
        } else {
            self.body = body.to_vec();
        }

        self.headers
            .insert("content-length".to_string(), format!("{}", self.body.len()));

        self
    }

    pub(crate) fn into_bytes(self) -> Result<Vec<u8>> {
        let mut buffer = Vec::new();
        write!(buffer, "{} {}", self.http_version, self.status.to_string())?;
        write!(buffer, "\r\n")?;
        for (key, value) in self.headers.into_iter() {
            buffer.write(key.as_bytes())?;
            write!(buffer, ": ")?;
            buffer.write(value.as_bytes())?;
            write!(buffer, "\r\n")?;
        }
        write!(buffer, "\r\n")?;
        buffer.write(&self.body)?;

        Ok(buffer)
    }
}
