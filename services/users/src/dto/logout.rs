use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    pub successful: bool,
}

impl LogoutResponse {
    pub fn success() -> Self {
        LogoutResponse { successful: true }
    }
}
