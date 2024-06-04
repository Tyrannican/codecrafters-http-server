use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub(crate) enum HttpMethod {
    Get,
    Invalid,
}

pub(crate) enum HttpStatus {
    OK,
    NotFound,
}

impl HttpStatus {
    pub(crate) fn to_code(self) -> u16 {
        match self {
            Self::OK => 200,
            Self::NotFound => 404,
        }
    }
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct HttpRequest {
    pub(crate) line: RequestLine,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Vec<u8>,
}

#[derive(Debug, Clone)]
pub(crate) struct RequestLine {
    pub(crate) method: HttpMethod,
    pub(crate) url: String,
    pub(crate) version: String,
}

impl RequestLine {
    fn new(req: &[u8]) -> Result<Self> {
        let req = String::from_utf8(req.to_vec())?;
        let line = req.split(' ').collect::<Vec<&str>>();

        Ok(Self {
            method: HttpMethod::from(line[0]),
            url: line[1].to_string(),
            version: line[2].to_string(),
        })
    }
}

impl HttpRequest {
    pub(crate) fn new(req: &[u8]) -> Result<Self> {
        let request = req
            .split(|&b| b == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
            .collect::<Vec<&[u8]>>();

        let raw_headers = &request[1..request.len() - 2];
        let body = match request.last() {
            Some(body) => body.to_vec(),
            None => vec![],
        };

        let mut headers = HashMap::new();
        for header in raw_headers {
            let header = String::from_utf8(header.to_vec())?;
            let Some((key, value)) = header.split_once(": ") else {
                anyhow::bail!("invalid header");
            };

            headers.insert(key.to_string(), value.to_string());
        }

        Ok(Self {
            line: RequestLine::new(request[0])?,
            headers,
            body: body.to_vec(),
        })
    }
}
