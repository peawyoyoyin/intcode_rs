use std::fs;

pub mod log;

pub fn load_code_from_file(file_path: &str) -> Vec<isize> {
    let content = fs::read_to_string(file_path).expect(&format!("error while reading {file_path}"));
    content
        .split(',')
        .map(|token| {
            token
                .parse::<isize>()
                .expect(&format!("malformatted input code"))
        })
        .collect()
}
