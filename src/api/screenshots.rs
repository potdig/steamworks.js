use napi_derive::napi;

#[napi]
pub mod screenshots {
    use napi::Error;
    use std::path::Path;
    #[napi]
    pub fn add_screenshot_to_library(
        filename: String,
        thumbnail_filename: Option<String>,
        width: i32,
        height: i32,
    ) -> Result<u32, Error> {
        let client = crate::client::get_client();
        client.screenshots().add_screenshot_to_library(
            Path::new(&filename),
            thumbnail_filename.as_ref().map(|f| Path::new(f)),
            width,
            height,
        )
        .map_err(|e| Error::from_reason(e.to_string()))
    }
}