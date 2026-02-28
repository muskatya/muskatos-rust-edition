use std::{io::{self, Write}};

pub fn shell(users: &mut Vec<(String, String)>, _hostname: &str, user: &mut String) {
    println!("\nYou have successfully entered to the system!");
    println!("Type 'help' to get some help.\n");

    loop {
        let curr_user: &str = user.trim();
        let mut user_input: String = String::new();

        print!("{curr_user} [/] \x1b[31m=>\x1b[0m ");
        io::stdout().flush().expect("Cannot flush :(");
        io::stdin().read_line(&mut user_input).expect("Cannot read line :(");
        let user_input: &str = user_input.trim();
        if user_input.is_empty() {
            continue;
        }

        let mut parts = user_input.splitn(2, ' ');
        let command: &str = parts.next().unwrap_or("");
        let args: &str = parts.next().unwrap_or("");

        if command.is_empty() {
            continue;
        }

        match command {
            "echo" => {
                if args.is_empty() {
                    println!();
                } else {
                    println!("{args}");
                }
            },
            "exit" => {
                println!("Shutting down the system..\n");
                break;
            },
            "help" => println!("commands:
    help - gives you some help
    echo - prints something
    exit - exits from the system
    version - shows current system version

    adduser - adds a new user
    rmuser - removes a user
    su - switches into another user
    passwd - changes user\'s password
    "),
            "adduser" => adduser(args, users),
            "su" => su(args, users, user),
            "version" => println!("MuskatOS Rust Edition Prototype 1.1"),
            "passwd" => passwd(args, users),
            "rmuser" => rmuser(args, users),
            _ => println!("Command '{command}' is invalid!"),
        }
    }
}

fn adduser(args: &str, users: &mut Vec<(String, String)>) {
    let mut parts = args.split(' ');
    let nuser: &str = parts.next().unwrap_or("");

    if nuser == "" {
        println!("Missing user to add");
    } else {
        print!("Please enter root password: ");
        io::stdout().flush().expect("Cannot flush :(");
        let mut rpass: String = String::new();
        io::stdin().read_line(&mut rpass).expect("Cannot read line :(");
        if rpass.trim() == users[0].1 {
            
            let mut matchus = false;
            for us in users.iter() {
                if nuser == us.0 {
                    matchus = true;
                    break;
                }
            }
            if matchus {
                println!("User already exists");
            } else {
                users.push((nuser.to_string(), String::from("")))
            }

        } else {
            println!("Incorrect root password")
        }
    }
}

fn su(args: &str, users: &mut Vec<(String, String)>, user: &mut String) {
    let mut parts = args.split(' ');
    let suser: &str = parts.next().unwrap_or("");

    if suser == "" {
        println!("Missing user to switch to");
    } else {
        print!("Please enter {}\'s password: ", suser);
        io::stdout().flush().expect("Cannot flush :(");
        let mut upass: String = String::new();
        io::stdin().read_line(&mut upass).expect("Cannot read line :(");
        let upass = upass.trim();
            
        let mut matchus = false;
        for us in users.iter() {
            if suser == us.0 {
                if upass == us.1 {
                    matchus = true;
                    break;
                }
            }
        }
        if !matchus {
            println!("User does not exist or you entered incorrect user password");
        } else {
            *user = suser.to_string();
        }
    }
}

fn passwd(args: &str, users: &mut Vec<(String, String)>) {
    let mut parts = args.split(' ');
    let puser: &str = parts.next().unwrap_or("");

    if puser == "" {
        println!("Missing user to passwd");
    } else {
        print!("Please enter root password: ");
        io::stdout().flush().expect("Cannot flush :(");
        let mut rpass: String = String::new();
        io::stdin().read_line(&mut rpass).expect("Cannot read line :(");

        print!("Please enter user\'s password: ");
        io::stdout().flush().expect("Cannot flush :(");
        let mut npass: String = String::new();
        io::stdin().read_line(&mut npass).expect("Cannot read line :(");

        if rpass.trim() == users[0].1 {
            
            for us in users.iter_mut() {
                if puser == us.0 {
                    us.1 = npass.trim().to_string();
                    break;
                }
            }

        } else {
            println!("Incorrect root password")
        }
    }
}

fn rmuser(args: &str, users: &mut Vec<(String, String)>) {
    let mut parts = args.split(' ');
    let ruser: &str = parts.next().unwrap_or("");

    if ruser == "" {
        println!("Missing user to remove");
    } else if ruser == "root" {
        println!("Cannot remove root user"); 
    } else {
        print!("Please enter root password: ");
        io::stdout().flush().expect("Cannot flush :(");
        let mut rpass: String = String::new();
        io::stdin().read_line(&mut rpass).expect("Cannot read line :(");
        if rpass.trim() == users[0].1 {     
        users.retain(|us| us.0 != ruser);

        } else {
            println!("Incorrect root password")
        }
    }
}
