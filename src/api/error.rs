use std::fmt::Display;

use actix_web::{error, http::StatusCode, HttpResponse};
/*
 enum MyEnum {
    Variant1,
    Variant2,
    Variant3,
}
impl MyEnum {
    fn as_str(&self) -> &'static str {
        match self {
            MyEnum::Variant1 => "Variant 1",
            MyEnum::Variant2 => "Variant 2",
            MyEnum::Variant3 => "Variant 3",
        }
    }
}
impl std::string::ToString for MyEnum {
    fn to_string(&self) -> String {
        self.as_str().to_owned()
    }
}
impl std::fmt::Display for MyEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
 */

#[derive(Debug)]
pub enum ErrorResp {
    InternalError,
}
impl ErrorResp {
    fn as_str(&self) -> &'static str {
        match self {
            ErrorResp::InternalError => "Internal Server Error",
        }
    }
}
impl Display for ErrorResp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl error::ResponseError for ErrorResp {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).body(self.to_string())
    }
    fn status_code(&self) -> actix_web::http::StatusCode {
        match *self {
            ErrorResp::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
