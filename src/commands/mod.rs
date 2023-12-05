pub mod url;
pub mod file;

pub use url::change_url;
pub use url::show_url;
pub use url::reset_url;

pub use file::upload;
pub use file::construct_url;

// consts
pub const UPLOAD_DEST: &str = "store/media1/";
pub const ATTACHMENTS_DOWNLOAD: &str = "attachments/download/";
pub const ATTACHMENTS_VIEW: &str = "attachments/view/";
