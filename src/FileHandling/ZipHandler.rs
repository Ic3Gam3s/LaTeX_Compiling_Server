use axum::extract::Multipart;
use axum::http::StatusCode;

#[derive(Clone)]
pub struct ZipHandler {}

impl ZipHandler {
    pub fn new() -> ZipHandler {
        ZipHandler {}
    }

    pub fn open_zip_and_load_into_temp(&self) {}
}
