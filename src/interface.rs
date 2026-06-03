use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about=None)]
pub(crate) struct Interface {
    #[arg(short, long)]
    pub(crate) json: bool, //output in a json format

    /// provide an override for the %LOCALAPPDATA% path in the event that environment variables cannot be relied on
    #[cfg(windows)]
    #[arg(short = 'L', long)]
    pub(crate) override_localappdata_path: Option<String>,

    /// provide an override for the %APPDATA% path in the event that environment variables cannot be relied on
    #[cfg(windows)]
    #[arg(short = 'A', long)]
    pub(crate) override_appdata_path: Option<String>,
}
