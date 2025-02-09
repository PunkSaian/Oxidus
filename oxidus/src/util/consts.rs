pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[allow(dead_code)]
pub const OXIDE_LOGO: &[u8] = include_bytes!("../../assets/oxide-logo.bmp");
#[allow(dead_code)]
pub const OXIDE_LOGO48: &[u8] = include_bytes!("../../assets/oxide-logo.bmp");

pub fn info_string() -> String {
    format!("{NAME} v{VERSION} by {AUTHORS}")
}
