use std::io::{self, Write};

pub fn prompt(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn prompt_password(label: &str) -> String {
    print!("{}: ", label);
    io::stdout().flush().unwrap();
    // TODO: use rpassword for hidden input
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
