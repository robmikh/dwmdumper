use std::path::Path;

use windows::{
    core::{Error, Result, HSTRING},
    Win32::Storage::FileSystem::GetFullPathNameW,
};

#[derive(Copy, Clone, Debug)]
pub enum ExecutionMode {
    Immediate,
    KeyCombo,
}

#[derive(Clone, Debug)]
pub struct Args {
    pub output_path: String,
    pub mode: ExecutionMode,
}

impl Args {
    pub fn from_args() -> Result<Self> {
        let mut args: Vec<_> = std::env::args().skip(1).collect();

        let mut result = Self {
            output_path: "dwmdump.dmp".to_owned(),
            mode: ExecutionMode::KeyCombo,
        };
        if args
            .iter()
            .position(|arg| arg == "/?" || arg == "-?" || arg == "/help" || arg == "-help")
            .is_some()
        {
            println!(
                r#"{name}
v{version}
{description}

USAGE:
    {name} [OPTIONS] [destination]
    
OPTIONS:
    /help            Print help information
    /immediate       Capture a dump immediately instead of using the key combo
    destination      Destination file to save the dump (default: dwmdump.dmp)
"#,
                name = env!("CARGO_PKG_NAME"),
                version = env!("CARGO_PKG_VERSION"),
                description = env!("CARGO_PKG_DESCRIPTION")
            );
            std::process::exit(0)
        }

        if let Some(index) = args
            .iter()
            .position(|arg| arg == "-immediate" || arg == "/immediate")
        {
            result.mode = ExecutionMode::Immediate;
            args.remove(index);
        }

        if let Some(output_path) = args.last() {
            if !validate_path(output_path) {
                println!("Invalid path! Expecting dmp file.");
                std::process::exit(1)
            }
            result.output_path = get_full_path_name(output_path)?;
        } else {
            result.output_path = get_full_path_name(&result.output_path)?;
        }

        Ok(result)
    }
}

fn validate_path<P: AsRef<Path>>(path: P) -> bool {
    let path = path.as_ref();
    let mut valid = true;
    if let Some(extension) = path.extension() {
        if extension != "dmp" {
            valid = false;
        }
    } else {
        valid = false;
    }
    valid
}

fn get_full_path_name(input_path: &str) -> Result<String> {
    let input_path = HSTRING::from(input_path);
    let mut buffer = Vec::<u16>::new();
    let len = unsafe { GetFullPathNameW(&input_path, &mut buffer, std::ptr::null_mut()) };
    if len == 0 {
        return Err(Error::from_win32());
    }
    buffer.resize(len as usize, 0);
    let len = unsafe { GetFullPathNameW(&input_path, &mut buffer, std::ptr::null_mut()) };
    if len == 0 {
        return Err(Error::from_win32());
    }
    buffer.resize(len as usize, 0);
    let full_path = String::from_utf16(&buffer).unwrap();
    Ok(full_path)
}
