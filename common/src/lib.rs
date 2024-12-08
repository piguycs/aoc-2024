use std::fs::read_to_string;

pub fn get_input(day: usize, input_file: &str) -> String {
    let root = format!("{}/..", env!("CARGO_MANIFEST_DIR"));
    let file_name = format!("{}/day-{:02}/{}", root, day, input_file);

    read_to_string(file_name).expect("Could not open given file")
}
