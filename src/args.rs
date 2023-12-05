use clap::{ Args, Parser, Subcommand };

// macros
#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct FileDartArgs {
    #[clap(subcommand)]
    /// command to run
    pub command_name: CommandEnums,
}

#[derive(Debug, Subcommand)]
pub enum CommandEnums {
    /// API commands
    Url(UrlCommand),

    /// File commands
    File(FileCommand),
}

#[derive(Debug, Args)]
pub struct UrlCommand {
    #[clap(subcommand)]
    pub command: UrlSubCommand,
}

#[derive(Debug, Args)]
pub struct FileCommand {
    #[clap(subcommand)]
    pub command: FileSubCommand,
}

#[derive(Debug, Subcommand)]
pub enum UrlSubCommand {
    /// Show the current API URL
    Show,

    /// Reset the API URL to default
    Reset,

    /// Change the API URL
    Change(ChangeUrl),
}

#[derive(Debug, Subcommand)]
pub enum FileSubCommand {
    /// Upload a file to the cloud
    Upload(UploadFile),

    /// Construct a download / view URL for a fileId
    ConstructUrl(ConstructFileUrl),
}

// command values
#[derive(Debug, Args)]
pub struct ChangeUrl {
    pub url: String,
}

#[derive(Debug, Args)]
pub struct UploadFile {
    pub location: String,
}

#[derive(Debug, Args)]
pub struct ConstructFileUrl {
    pub file_id: String,
}
