use std::fs;
pub const LINE_ENDING: &'static str = "\n";

pub fn read_input_file_line_separated(filename: &str) -> Vec<String> {
    read_input_file(filename, LINE_ENDING)
}

pub fn read_input_file(filename: &str, separator: &str) -> Vec<String> {
    read_file(filename).split(separator).map(String::from).collect::<Vec<_>>()
}

pub fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(content) => {
            content.trim().to_string()
        },
        Err(err) => panic!("{}", err.to_string())
    }
}
