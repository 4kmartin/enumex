mod common;
mod error;
mod interface;
#[cfg(windows)]
mod windows;

fn main() -> error::Result<()> {
    use clap::Parser;
    let args = interface::Interface::parse();
    if !args.json {
        return common::get_extensions();
    };
    Ok(())
}
