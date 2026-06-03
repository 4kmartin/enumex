use clap::Parser;

mod common;
mod error;
mod interface;
#[cfg(windows)]
mod windows;

fn main() -> error::Result<()> {
    let args = interface::Interface::parse();
    return common::get_extensions(args);
}
