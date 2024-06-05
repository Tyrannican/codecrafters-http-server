use anyhow::Result;
use std::{collections::HashMap, io::Write};

use super::HttpStatus;

pub(crate) struct HttpResponse {
    http_version: String,
    status: HttpStatus,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpResponse {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn header(mut self, header: (&str, &str)) -> Self {
        self.headers
            .insert(header.0.to_string(), header.1.to_string());
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
        self.body = body.to_vec();
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
