use napi_derive::napi;

#[napi]
pub mod screenshots {
    #[napi]
    pub fn trigger_screenshot() {
        let client = crate::client::get_client();
        client.screenshots().trigger_screenshot();
    }
}
