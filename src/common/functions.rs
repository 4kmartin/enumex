use crate::common::{Browser, FilePath, Files, Publisher};
use crate::error::{ExtError, Result, convert_result};
use serde_json::Value;
use std::{fs::read_dir, io::Read, path::Path};

#[cfg(windows)]
use crate::windows::{get_chromium_root, get_firefox_root};

fn print_extensions(extensions: Vec<Result<String>>) -> Result<()> {
    for ext in extensions {
        println!("{}", ext?.replace("\"", ""));
    }
    Ok(())
}

fn chromium_ext_dir(root: FilePath) -> Result<Files> {
    let mut elems = Files::new();
    for e in ls(&root)? {
        elems.append(&mut ls(&e)?);
    }
    Ok(elems)
}
fn chromium_manifest(ext_dir: &FilePath) -> Result<String> {
    if get_locl(ext_dir).is_dir() {
        let path = add_to_path(ext_dir, "_locales\\en\\messages.json");
        let msgs = read_json(&path)?;
        // println!("{:#?}", msgs);
        Ok(msgs["extname"]["message"].to_string())
    } else {
        let path = add_to_path(ext_dir, "manifest.json");
        let mani = read_json(&path)?;
        Ok(mani["name"].to_string())
    }
}

fn read_json(file: &Path) -> Result<Value> {
    let mut content = String::new();
    let mut file = convert_result(std::fs::File::open(file), ExtError::FileNotFound)?;
    convert_result(file.read_to_string(&mut content), ExtError::FileNotFound)?;
    convert_result(
        serde_json::from_str(&content.to_lowercase()),
        ExtError::JsonParsingError,
    )
}

fn get_locl(ext_dir: &Path) -> FilePath {
    add_to_path(ext_dir, "_locales")
}

fn add_to_path(path: &Path, added: &str) -> FilePath {
    let mut new = path.to_path_buf();
    new.push(added);
    new.into_boxed_path()
}

fn ls(p: &Path) -> Result<Files> {
    Ok(convert_result(read_dir(p), ExtError::DirectoryNotFound)?
        .filter_map(|r| r.ok())
        .map(|e| e.path().into_boxed_path())
        .collect::<Files>())
}

fn get_chromium_ext(path_override: &Option<String>) -> Result<Vec<Result<String>>> {
    let mut output = Vec::new();
    for (publisher, browser) in [
        (Publisher::Microsoft, Browser::Edge),
        (Publisher::Google, Browser::Chrome),
    ] {
        let root = get_chromium_root(publisher, browser, path_override)?;
        if let Ok(outs) = chromium_ext_dir(root) {
            output.append(
                &mut outs
                    .iter()
                    .map(|ext_dir| chromium_manifest(ext_dir))
                    .collect::<Vec<Result<String>>>(),
            )
        }
    }
    Ok(output)
}

fn firefox_ext_paths(root: FilePath) -> Result<Files> {
    Ok(ls(&root)?
        .iter()
        .filter(|e| e.join("extensions.json").exists())
        .map(|e| e.join("extensions.json").into_boxed_path())
        .collect::<Files>())
}

fn firefox_ext_file(file: &Path) -> Result<Vec<String>> {
    match read_json(file)?["addons"].as_array() {
        Some(v) => Ok(v
            .iter()
            .filter(|x| x["type"] == "extension")
            .map(|x| x["defaultlocale"]["name"].to_string())
            .collect::<Vec<String>>()),
        None => Err(ExtError::JsonParsingError(
            "failed to enumerate \"addons\" field".to_string(),
        )),
    }
}

fn get_firefox_ext(path_override: &Option<String>) -> Result<Vec<Result<String>>> {
    let mut out: Vec<Result<String>> = Vec::new();
    if let Ok(ext_paths) = firefox_ext_paths(get_firefox_root(path_override)?) {
        for file in ext_paths {
            let mut exts = firefox_ext_file(&file)?
                .iter()
                .map(|x| Ok(x.to_string()))
                .collect::<Vec<Result<String>>>();
            out.append(&mut exts);
        }
    };
    Ok(out)
}

pub(crate) fn get_extensions(args: crate::interface::Interface) -> Result<()> {
    let mut extensions = get_chromium_ext(&args.override_localappdata_path)?;
    extensions.append(&mut get_firefox_ext(&args.override_appdata_path)?);
    if args.json {
        print_extensions_json(extensions)
    } else {
        print_extensions(extensions)
    }
}

fn print_extensions_json(extensions: Vec<Result<String>>) -> Result<()> {
    let mut out = Vec::new();
    for ext in extensions {
        if let Ok(name) = ext {
            let json = serde_json::json!({"name": name});
            out.append(&mut vec![json])
        }
    }
    println!("{}", serde_json::json!({"extensions": out}).to_string());

    Ok(())
}
