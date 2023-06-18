use std::io::{self, Write};

pub fn read_input(prompt: &str) -> io::Result<String> {
    let mut usr_input = String::new();
    print!("{}", prompt);
    io::stdout().flush()?;
    io::stdin().read_line(&mut usr_input)?;
    Ok(usr_input.trim().to_string())
}