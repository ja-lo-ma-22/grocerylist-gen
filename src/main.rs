use std::env;
use std::io;

mod grocery_database;

fn main() {
    println!("Starting grocerylist-gen...");
    
    let mut arguments: Vec<String> = env::args().collect();

    let _program_name = arguments.remove(0);
    
    if arguments.is_empty() {
        interactive_mode();
    } else {
        cli_mode();
    }
}

fn interactive_mode() {
    loop {
        let mut command = String::new();
        io::stdin()
            .read_line(&mut command)
            .expect("WTF???");

        match command.as_str() {
            "add" => { 
                add_item();
                println!("Done.");
            }
            "items" => {
                print_list("items");
                println!("");
            }
            "prices" => { print_list(""); }
            "total" => {}
            "all" => {}
            "help" => {}
            "exit" | "quit" => break;
            _ => { println!("Bad command.") }

        }
    }
}

fn cli_mode() {}

fn add_item(name: String, price: f32) {}

fn print_list(kind: String) {}


fn parse_args(unparsed: Vec<String>) -> Vec<String> {
    println!("Parsing arguments...");
    unparsed
}

fn interactive_questioning(question: String) -> String {
    let mut answer = String::new();
    println!("{question}");
    io::stdin()
        .read_line(&mut answer)
        .expect("WTF?!");
    answer
}
