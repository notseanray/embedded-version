use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::fs::{create_dir_all, File};
use std::io::{BufWriter, Read, Write};
use std::path::Path;
use std::process::Command;

fn same_content_as(path: &Path, content: &str) -> Result<bool, Box<dyn Error>> {
    let mut f = File::open(path)?;
    let mut current = String::new();
    f.read_to_string(&mut current)?;

    Ok(current == content)
}

fn git_describe() -> Option<String> {
    Command::new("git")
        .args(["describe", "--tags", "--always"])
        .output()
        .ok()
        .and_then(|out| {
            std::str::from_utf8(&out.stdout[..])
                .map(str::trim)
                .map(str::to_owned)
                .ok()
        })
}

pub fn version() -> Result<(), Box<dyn Error>> {
    let path = env::var_os("OUT_DIR").expect("expected OUR_DIR");
    let path: &Path = path.as_ref();

    create_dir_all(path)?;

    let path = path.join("version.rs");

    let version_number = read_to_string("Cargo.toml")?.lines().filter_map(|x| {
        if x.starts_with("version = \"") {
            Some(x.replace('\"', ""))
        } else {
            None
        }
    }).collect::<Vec<String>>().pop().expect("no version found in Cargo.toml");

    let content = if let Some(describe) = git_describe() {
        format!(
            "pub(crate) static VERSION: &str = \"{version_number}, {describe}\";\n"
        )
    } else {
        "pub(crate) static VERSION: &str = \"no version could be found\";\n".to_owned()
    };

    let is_fresh = if path.exists() {
        same_content_as(&path, &content)?
    } else {
        false
    };

    if !is_fresh {
        let mut file = BufWriter::new(File::create(&path)?);

        write!(file, "{}", content)?;
    }
    Ok(())
}
