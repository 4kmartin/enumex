use std::fmt::Display;
use std::path::Path;

pub(crate) enum Browser {
    Chrome,
    Edge,
}

impl Display for Browser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Browser::Chrome => write!(f, "Chrome"),
            Browser::Edge => write!(f, "Edge"),
        }
    }
}

pub(crate) enum Publisher {
    Google,
    Microsoft,
}

impl Display for Publisher {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Publisher::Google => write!(f, "Google"),
            Publisher::Microsoft => write!(f, "Microsoft"),
        }
    }
}

pub(crate) type FilePath = Box<Path>;
pub(crate) type Files = Vec<FilePath>;
