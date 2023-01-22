use ::std::str::FromStr;
use std::num::ParseIntError;

pub enum Method {
    GET,
    POST,
    PATCH,
    DELETE,
    OPTION,
    TRACE,
    PUT,
    CONNECT,
    HEAD,
}

impl FromStr for Method {
    type Err = MethodError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Self::GET),
            "POST" => Ok(Self::POST),
            "PATCH" => Ok(Self::PATCH),
            "DELETE" => Ok(Self::DELETE),
            "OPTION" => Ok(Self::OPTION),
            "TRACE" => Ok(Self::TRACE),
            "PUT" => Ok(Self::PUT),
            "CONNECT" => Ok(Self::CONNECT),
            "HEAD" => Ok(Self::HEAD),
            _ => Err(MethodError),
        };
        unimplemented!()
    }
}

pub struct MethodError;
