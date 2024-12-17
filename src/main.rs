use std::fs::rename;

#[derive(Debug)]
enum Error {
    MissingArguments(String),
    ReadError(std::io::Error),
}
fn main() -> Result<(), Error> {
    let from = std::env::args().nth(1).ok_or(Error::MissingArguments("Missing args".to_string()))?;
    let to = std::env::args().nth(2).ok_or(Error::MissingArguments("Missing args".to_string()))?;
    println!("{from}, {to}");
    rename(from, to).map_err(|err|Error::ReadError(err))?;
    Ok(())

}
