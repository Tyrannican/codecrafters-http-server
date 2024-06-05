pub(crate) mod request;
pub(crate) mod response;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum HttpMethod {
    Get,
    Post,
    Invalid,
}

pub(crate) enum HttpStatus {
    OK,
    NotFound,
    BadRequest,
}

impl HttpStatus {
    pub(crate) fn to_string(self) -> String {
        match self {
            Self::OK => "200 OK".to_string(),
            Self::NotFound => "404 Not Found".to_string(),
            Self::BadRequest => "400 Bad Request".to_string(),
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
