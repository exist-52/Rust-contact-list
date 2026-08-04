use std::{env::{self}, fs::File, io::Write};

fn main() {
    let args: Vec<String> = env::args().collect();
    let contact = collect(&args);
    if contact.operation == "-a" {
        add_contact(contact);
    }
    else if contact.operation == "-f" {
        fetch_contact(contact.name);
    }
    else{
        panic!("possible operations:
            -a: add a number
            -f: fetch a number
            ");
    }
}

struct Contact {
    operation: String,
    name: String,
    number: String,
    path: String,
}

impl Contact {
    fn build(operation: String, name: String, number: String, path: String) -> Self {
        Contact { operation, name, number, path }
    }
}

fn collect(args: &Vec<String>) -> Contact {
    if args.len() < 4 {
        panic!("Usage: cargo run -a name number file_path");
    }

    Contact::build(args[0].to_string(), args[1].to_string(), args[2].to_string(), args[3].to_string())
}

fn write_contact(mut file: File, name: String, number: String) {
    let person = format!("{name} : {number}");
    file.write(person.as_bytes());
}

fn add_contact(contact: Contact) {
    let file = File::open(&contact.path);
    match file{
        Ok(file) => {
            write_contact(file, contact.name, contact.number);
        }
        Err(..) => {
            let create = File::create(&contact.path);
            match create {
                Ok(file) => {
                    write_contact(file, contact.name, contact.number);
                    println!("{} was not found. path created instead", contact.path);
                }
                Err(e)=> {
                panic!("Could not create file: {}", e);
            }
            }

        }
    }
}
fn fetch_contact(_name:String){
    unimplemented!()
}
