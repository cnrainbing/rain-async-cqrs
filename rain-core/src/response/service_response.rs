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

    ///
    /// HttpResponse::Ok().json(ServiceResponse::success(data.unwrap()))
    ///
    pub fn success(data: T) -> ServiceResponse<T> {
        ServiceResponse {
            code: ErrorCode::OK.as_u32(),
            message: ErrorCode::OK.as_str(),
            data,
        }
    }

    pub fn failure_msg_str(code: u32, message: &'static str) -> ServiceResponse<&'static str> {
        ServiceResponse {
            code,
            message,
            data: "",
        }
    }

    pub fn failure_str(error_code: ErrorCode) -> ServiceResponse<&'static str> {
        ServiceResponse {
            code: error_code.as_u32(),
            message: error_code.as_str(),
            data: "",
        }
    }

    pub fn failure_e_msg(error_code: ErrorCode, message: &'static str) -> ServiceResponse<&'static str> {
        ServiceResponse {
            code: error_code.as_u32(),
            message,
            data: "",
        }
    }
}