use anyhow::Result;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum HttpMethod {
    Get,
    Post,
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
            "PUT" => Self::Post,
            _ => Self::Invalid,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct HttpRequest {
    pub(crate) method: HttpMethod,
    pub(crate) url: String,
    pub(crate) version: String,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) body: Vec<u8>,
}

impl HttpRequest {
    pub(crate) fn new(req: &[u8]) -> Result<Self> {
        let request = req
            .split(|&b| b == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
            .collect::<Vec<&[u8]>>();

        let req_line = String::from_utf8(request[0].to_vec())?;
        let req_line = req_line.split(' ').collect::<Vec<&str>>();
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
            method: HttpMethod::from(req_line[0]),
            url: req_line[1].to_string(),
            version: req_line[2].to_string(),
            headers,
            body: body.to_vec(),
        })
    }
}
