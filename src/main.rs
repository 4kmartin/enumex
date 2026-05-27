mod common;
mod error;
#[cfg(windows)]
mod windows;

use crate::common::get_extensions;

fn main() -> error::Result<()> {
    get_extensions()
}
