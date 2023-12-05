use crate::file_functions::ConfigFile;

pub fn run_showurl() {
    let url = ConfigFile::get_url();
    println!("The current API URL is {}", url)
}
