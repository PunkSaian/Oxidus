pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[allow(dead_code)]
pub const OXIDE_LOGO_BMP: &[u8] = include_bytes!("../../assets/oxide-logo.bmp");
#[allow(dead_code)]
pub const OXIDE_LOGO_BMP_48: &[u8] = include_bytes!("../../assets/oxide-logo-48.bmp");
const OXIDE_LOGO_PNG: &[u8] = include_bytes!("../../assets/oxide-logo.png");

pub fn info_string() -> String {
    format!("{NAME} v{VERSION} by {AUTHORS}")
}
