use std::process;
use reqwest::blocking::Client;
use reqwest::blocking::multipart::Part;
use reqwest::blocking::multipart::Form;
use std::io::Read;
use mime_guess::from_path;

use crate::file_functions::ConfigFile;
use crate::TOKEN;
use super::super::UPLOAD_DEST;

use serde::Deserialize;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize)]
struct FileResponse {
    fileId: String,
    viewUrl: String,
    downloadUrl: String,
}

#[derive(Debug, Deserialize)]
struct ResponseFailed {
    response: u16,
    error: String,
}

pub fn run_upload_file(location: String) {
    // Read the file as bytes
    let mut file = ConfigFile::open_file(&location);

    // Read the file content into a buffer
    let mut file_content = Vec::new();
    if let Err(err) = file.read_to_end(&mut file_content) {
        eprintln!("Error reading file: {}", err);
        process::exit(1);
    }

    // Guess the MIME type based on the file content
    let mime_type: mime_guess::Mime = from_path(&location).first_or_octet_stream();
    let file_name: String = ConfigFile::get_filename(&location);

    // Create a reqwest Client
    let client = Client::new();

    // Build the multipart request
    let form = Form::new().part(
        "file",
        Part::bytes(file_content)
            .file_name(file_name)
            .mime_str(mime_type.as_ref())
            .unwrap_or_else(|err| {
                eprintln!("Error setting MIME type: {}", err);
                process::exit(1);
            })
    );

    // get the current url
    let api_url: String = ConfigFile::get_url();

    // Send the multipart/form-data request with authorization header
    let mut response = client
        .post(format!("{}{}", &api_url, &UPLOAD_DEST))
        .header("Authorization", format!("Bearer {}", TOKEN))
        .multipart(form)
        .send()
        .unwrap_or_else(|err| {
            eprintln!("Error sending request: {}", err);
            process::exit(1);
        });

    let mut body = String::new();
    // Read the response body as a string
    if let Err(err) = response.read_to_string(&mut body) {
        eprintln!("Error reading response body: {}", err);
        process::exit(1);
    }

    // Check if the request was successful
    if response.status().is_success() {
        let body: FileResponse = match serde_json::from_str(&body) {
            Ok(parsed_body) => parsed_body,
            Err(err) => {
                eprintln!("Error converting json in / y: {}", err);
                process::exit(1);
            }
        };

        // Access the extracted fields
        println!("File ID: {}", body.fileId);
        println!("View URL: {}", body.viewUrl);
        println!("Download URL: {}", body.downloadUrl);
    } else {
        let body: ResponseFailed = match serde_json::from_str(&body) {
            Ok(parsed_body) => parsed_body,
            Err(err) => {
                eprintln!("Error converting json in / x: {}", err);
                process::exit(1);
            }
        };

        println!("Request failed with status code: {}", body.response);
        println!("Error message: {}", body.error);
    }
}
