use anyhow::Result;
use std::{collections::HashMap, io::Write};

pub(crate) struct HttpResponse {
    status: String,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpResponse {
    pub(crate) fn new() -> Self {
        Self {
            status: String::new(),
            headers: HashMap::new(),
            body: Vec::new(),
        }
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
        buffer.write(self.status.as_bytes())?;
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
