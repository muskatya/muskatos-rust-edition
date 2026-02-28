use std::io::{self, Write};

mod shell;

fn main() {
    println!("MuskatOS Rust Edition Prototype 1.1\n");
    let hostname: &str = "MuskatOS";

    let mut users: Vec<(String, String)> = Vec::new();
    users.push((String::from("root"), String::from("root")));

    let mut user: String = String::new();

    login(&users, &mut user);

    shell::shell(&mut users, &hostname, &mut user);
}

fn login(users: &Vec<(String, String)>, user: &mut String) {
    while user.is_empty() {
        print!("User: ");
        io::stdout().flush().expect("Cannot flush :(");
        let mut input: String = String::new(); 
        io::stdin().read_line(&mut input).expect("Cannot read line :(");
        let input: &str = input.trim();

        print!("Password: ");
        io::stdout().flush().expect("Cannot flush :(");
        let mut input2: String = String::new(); 
        io::stdin().read_line(&mut input2).expect("Cannot read line :(");
        let input2: &str = input2.trim();

        let mut found: bool = false;

        for us in users {
            if us.0 == input {
                found = true;
                
                if us.1 == input2 {
                    *user = us.0.to_string();
                } else {
                    println!("Incorrect password, please try again");
                }
                break;
            }
        }

        if !found {
            println!("Incorrect username, please try again");
        }
    }
}
