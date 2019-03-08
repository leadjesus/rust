use std::io;

fn main() {
    //prompt for name
    println!("What is your name?");
    
    //create a string
    let mut ent_name = String::new();
    
    //accept input into ent_name
    io::stdin().read_line(&mut ent_name)
        .expect("failed to read line);
    //greetings
    print!("Welcome, ");
    print!("{}", ent_name);
