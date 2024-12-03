use napi_derive::napi;

#[napi]
pub mod screenshots {
    #[napi]
    pub fn hook_screenshots(hook: bool) {
        let client = crate::client::get_client();
        client.screenshots().hook_screenshots(hook);
    }

    #[napi]
    pub fn trigger_screenshot() {
        let client = crate::client::get_client();
        client.screenshots().trigger_screenshot();
    }
}
