pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const NAME: &str = env!("CARGO_PKG_NAME");
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

pub const OXIDUS_LOGO_BMP_48: &[u8] = include_bytes!("../../assets/oxidus-logo-48.bmp");
pub const OXIDUS_LOGO_PNG: &[u8] = include_bytes!("../../assets/oxidus-logo.png");
pub const OXIDUS_LOGO_TRANS_PNG: &[u8] = include_bytes!("../../assets/oxidus-logo-trans.png");
pub const OXIDUS_LOGO_TRANS_OUTLINED_PNG: &[u8] =
    include_bytes!("../../assets/oxidus-logo-trans-outlined.png");

pub fn info_string() -> String {
    format!("{NAME} v{VERSION} by {AUTHORS}")
}
