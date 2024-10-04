use std::io;
// A project to act like my password manager

// What I want to support
// 1. Getting stored passwords
// 2. Adding new passwords
// 3. Editing passwords (Not Sure)

fn main() {
    println!("This is the ultimate password manager!");

    loop {
        // Ask me what I want to do
        println!("------ MENU ----------");
        println!("1: Get a password");
        println!("2: Store a password:");
        println!("3: Exit");
        println!("----------------------");
        println!("Choose what you want to do");

        let mut choice = String::new();
        let stdin = io::stdin();
        stdin.read_line(&mut choice).expect("Could Not Get User Choice!");
        let choice: u8 = choice.trim().parse().expect("Could Not Parse Choice");

        if choice == 1 {
            print!("Enter the name of the site you want to get password of:");
            let mut site = String::new();
            stdin.read_line(&mut site).expect("Could Not Get Site Name");

            let password = get_password(&site);
            print!("Password is {password}");
        } else if choice == 2 {
            print!("Option 2");
        } else if choice == 3 {
            print!("Exiting! Thanks for using the program!\n");
            break
        }
    }
    store_password_file("test.txt");

    let site = "Test site";
    let password = get_password(site);
    println!("The password of {site} is {password}");
    let email = "test@test.com";

    store_new_password(site, email, password);
}

// A password to store file location of passwords file
fn store_password_file(file_path: &str) {
    println!("Password is stored in {file_path}");
}

// Get password of site
fn get_password(site_name: &str) -> &str {
    "Test Password"
}

// Store a new password
fn store_new_password(site: &str, email: &str, password: &str) {
    println!("Details site: {site} email: {email} password: {password} have been stored succesfully!");
}