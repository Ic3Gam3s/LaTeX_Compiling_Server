use serde::Deserialize;

#[derive(Deserialize)]
struct DownloadParams {
    fileName: String,
    offset: u64,
    chunkSize: usize,
}