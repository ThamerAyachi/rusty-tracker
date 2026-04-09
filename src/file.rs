use std::fs::File;
use std::io::Write;

const FILE_PATH: &'static str = "expense.json";

fn file() -> File {
    File::create(FILE_PATH).unwrap()
}

pub fn write_file(json: String) -> Result<(), std::io::Error> {
    file().write_all(json.as_bytes())
}

pub fn read_file() -> Result<String, std::io::Error> {
    std::fs::read_to_string(&FILE_PATH)
}
