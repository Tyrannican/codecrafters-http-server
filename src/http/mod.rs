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
    Created,
    NotFound,
    BadRequest,
    InternalServerError,
}

impl HttpStatus {
    pub(crate) fn to_string(self) -> String {
        match self {
            Self::OK => "200 OK".to_string(),
            Self::Created => "201 Created".to_string(),
            Self::NotFound => "404 Not Found".to_string(),
            Self::BadRequest => "400 Bad Request".to_string(),
            Self::InternalServerError => "500 Internal Server Error".to_string(),
        }
    }
}

impl From<&str> for HttpMethod {
    fn from(value: &str) -> Self {
        match value {
            "GET" => Self::Get,
            "POST" => Self::Post,
            _ => Self::Invalid,
        }
    }
}
