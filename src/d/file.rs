
pub fn read_input(file: &str) -> Result<Vec<String>, std::io::Error> {
    let contents = std::fs::read_to_string(file)?;
    let lines = contents.lines().map(|x| x.to_string()).collect();
    Ok(lines)
} 