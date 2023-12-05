use crate::file_functions::ConfigFile;

use crate::TOKEN;

use super::super::ATTACHMENTS_DOWNLOAD;
use super::super::ATTACHMENTS_VIEW;

pub fn run_construct_file_url(file_id: String) {
    // get the current url
    let api_url: String = ConfigFile::get_url();

    // construct the url
    println!("View URL: {}{}{}/{}", api_url, ATTACHMENTS_VIEW, file_id, TOKEN);
    println!("Download URL: {}{}{}/{}", api_url, ATTACHMENTS_DOWNLOAD, file_id, TOKEN)
}
