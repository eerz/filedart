// deps
// use std::env;

// modules
mod file_functions;
mod commands;
mod args;

// usages
use args::FileDartArgs;
use clap::Parser;

pub const PROGRAM_NAME: &str = "filedart";
pub const DEFAULT_API_URL: &str = "http://gr3.techstar.host:55582/";
pub const TOKEN: &str = "jeaRbskjaadZaVqTKy";

// enum defining each command
pub use args::CommandEnums;
pub use args::UrlSubCommand;
pub use args::FileSubCommand;

fn main() {
    let args: FileDartArgs = FileDartArgs::parse();

    match args.command_name {
        CommandEnums::Url(url_command) =>
            match url_command.command {
                UrlSubCommand::Show => {
                    commands::show_url::run_showurl();
                }
                UrlSubCommand::Change(change_url) => {
                    commands::change_url::run_changeurl(change_url.url);
                }

                UrlSubCommand::Reset => {
                    commands::reset_url::run_reseturl();
                }

                // more url commands
            }

        CommandEnums::File(upload_command) =>
            match upload_command.command {
                FileSubCommand::Upload(upload_file) => {
                    commands::upload::run_upload_file(upload_file.location)
                }

                FileSubCommand::ConstructUrl(construct_file_url) => {
                    commands::construct_url::run_construct_file_url(construct_file_url.file_id)
                }

                // more file commands
            }
    }
}
