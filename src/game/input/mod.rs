use std::io::Write;
use std::io;

pub fn get_string(before_input: &str) -> String {
    let mut input: String;
    input =  String::new();
    print!("{}: ", before_input);
    io::stdout().flush();
    io::stdin().read_line(&mut input);
    return input.trim().to_string();
}

pub fn get_number(before_input: &str) -> i32 {
    let mut input_text = String::new();
    let mut done = false;
    let mut number: i32 = 0;
    while !done {
        input_text = String::from("");
        print!("{}: ", before_input);
        io::stdout().flush();
        io::stdin()
        .read_line(&mut input_text)
        .expect("Qualcosa è andato molto male...");

        let trimmed = input_text.trim();
        match trimmed.parse::<i32>() {
            Ok(i) => {
                number = i;
                done = true;
            },
            Err(_) => println!("Devi darmi un numero..."),
        };
    }
    return number;
}