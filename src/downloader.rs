use std::path::Path;
use std::{fmt::Display, fs};

use std::io::{Error as OsError, ErrorKind};
use ureq::Error as InternetError;

const BASE_URL: &str = "https://adventofcode.com";
const STORAGE: &str = "input";
const API_KEY_PATH: &str = "API_KEY";

#[derive(Debug)]
pub enum InputGetError {
    DownloadError(InternetError),
    WriteError(OsError),
}

type AoCResult<T> = std::result::Result<T, InputGetError>;

impl From<InternetError> for InputGetError {
    fn from(value: InternetError) -> Self {
        Self::DownloadError(value)
    }
}

impl From<OsError> for InputGetError {
    fn from(value: OsError) -> Self {
        Self::WriteError(value)
    }
}

impl Display for InputGetError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for InputGetError {}

pub fn get_input(day: u32, year: u32) -> AoCResult<String> {
    let file_name = format!("{}/{}/{}.txt", STORAGE, year, day);
    let data = match fs::read_to_string(&file_name) {
        Ok(data) => data,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => {
                let data = download(day, year)?;
                save(&data, &file_name)?;
                data
            }
            _ => return Err(InputGetError::WriteError(e)),
        },
    };
    Ok(data.trim().to_string())
}

fn save(data: &str, name: &str) -> AoCResult<()> {
    let path = Path::new(&name);

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(path, data)?;
    Ok(())
}

fn download(day: u32, year: u32) -> AoCResult<String> {
    println!("Downloading input data...");
    let input = ureq::get(&format!("{}/{}/day/{}/input", BASE_URL, year, day))
        .set(
            "Cookie",
            &format!("session={}", fs::read_to_string(API_KEY_PATH)?),
        )
        .call()?
        .into_string()?;
    Ok(input)
}
