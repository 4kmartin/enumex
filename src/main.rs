use clap::Parser;

mod common;
mod error;
mod interface;

#[cfg(windows)]
mod windows;

#[cfg(target_os = "linux")]
mod linux;

#[cfg(target_os = "macos")]
mod macos;

fn main() -> error::Result<()> {
    let args = interface::Interface::parse();
    return common::get_extensions(args);
}
