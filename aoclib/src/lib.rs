pub mod lib {
    pub fn read_data(file_name: &str) -> Result<String, std::io::Error> {
        let contents = match std::fs::read_to_string(file_name) {
            Ok(lines) => lines,
            Err(err) => {
                println!("error reading '{}' data: {}", file_name, err);
                return Err(err);
            }
        };
        return Ok(contents);
    }
}
