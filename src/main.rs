use std::fs::rename;

#[derive(Debug)]
enum AllError {
    MissingArguments,
    ReadError(std::io::Error),
    NoMatches(()),
}
impl From<std::io::Error> for AllError{
    fn from(value: std::io::Error) -> Self {
        AllError::ReadError(value)
    }
}
        
fn main() -> Result<(), AllError> {
    let main_regex = regex::RegexBuilder::new(r"(?:ep|episode)?\s*(\d{1,3})[^0-9]").case_insensitive(true).build().unwrap();
    let current_folder = std::fs::read_dir("./data").unwrap();
    let from = std::env::args().nth(1).ok_or(AllError::MissingArguments)?;
    let to = std::env::args().nth(2).ok_or(AllError::MissingArguments)?;
    println!("{from}, {to}");
    for file in current_folder {
        let test = main_regex.find(&file.unwrap().file_name().to_str().unwrap()).ok_or(AllError::NoMatches(()))?.as_str().to_string();
        println!("{}",&test);
    }
    rename(from, to)?;
    Ok(())

}
