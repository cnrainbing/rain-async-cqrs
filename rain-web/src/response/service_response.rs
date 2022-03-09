use serde::Serialize;
use crate::error::ErrorCode;

#[derive(Serialize)]
pub struct ServiceResponse<T> {
    pub code: u32,
    pub message: &'static str,
    pub data: T,
}

impl<T> ServiceResponse<T> {
    ///
    /// HttpResponse::Ok().json(ServiceResponse::<&'static str>::default())
    ///
    pub fn default() -> ServiceResponse<&'static str> {
        ServiceResponse {
            code: ErrorCode::OK.as_u32(),
            message: ErrorCode::OK.as_str(),
            data: "",
        }
    }

    /// 例子
    /// HttpResponse::Ok().json(ServiceResponse::<&'static str>::default(""))
    /// HttpResponse::Ok().json(ServiceResponse::<Vec<u8>>::default(Vec::new()))
    ///
    pub fn default_t(data: T) -> ServiceResponse<T> {
        ServiceResponse {
            code: ErrorCode::OK.as_u32(),
            message: ErrorCode::OK.as_str(),
            data,
        }
    }

    pub fn success(data: T) -> ServiceResponse<T> {
        ServiceResponse {
            code: ErrorCode::OK.as_u32(),
            message: ErrorCode::OK.as_str(),
            data,
        }
    }
}