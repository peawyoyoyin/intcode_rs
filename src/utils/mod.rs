use std::fs;

pub fn load_code_from_file(file_path: &str) -> Vec<usize> {
  let content = fs::read_to_string(file_path).expect(&format!("error while reading {file_path}"));
  content
    .split(',')
    .map(|token| {
      token
        .parse::<usize>()
        .expect(&format!("malformatted input code"))
    })
    .collect()
}
