use napi_derive::napi;

#[napi]
pub mod screenshots {
    use napi::Error;
    use steamworks::screenshots::ScreenshotHandle;
    use steamworks::screenshots::ScreenshotLibraryAddError;
    use std::path::Path;
    #[napi]
    pub fn add_screenshot_to_library(
        filename: String,
        thumbnail_filename: Option<String>,
        width: i32,
        height: i32
    ) -> Result<ScreenshotHandle, Error> {
        let client = crate::client::get_client();
        client.screenshots().add_screenshot_to_library(
            Path::new(&filename),
            thumbnail_filename.map(|f| Path::new(&f)),
            width,
            height
        ).map_err(|e| Error::from_reason(e.to_string()))
    }
}