use axum::extract::State;
use axum::response::IntoResponse;

use crate::FileHandling::ZipHandler::ZipHandler;

#[derive(Clone)]
pub struct CompilingHandler {
    zipHandler: ZipHandler,
}

impl CompilingHandler {
    pub fn new(zh: ZipHandler) -> CompilingHandler {
        return CompilingHandler {
            zipHandler: zh,
        };
    }

    pub async fn compile(State(handler): State<CompilingHandler>) -> impl IntoResponse {}
}
