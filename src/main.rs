use std::{env::{self}, fs::{File, OpenOptions}, io::{BufRead, BufReader, Write}};

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let operation = &args[0];
    if operation == "-a" {
        if args.len() != 4 {
            panic!("there must be 4 arguments: -a [name] [contact number or email] [file_path]");
        }
        let contact = Contact::build(&args);
        add_contact(contact);
    }
    else if operation == "-f" {
        if args.len() != 3 {
            panic!("there must be 3 arguments: -f [name] [file_path]");
        }
        let person = Person::build(&args);
        fetch_contact(person.name, person.path);
    }
    else{
         // println!("you gave {} as name", contact.name);
         // println!("you gave {} as number", contact.number);
         // println!("you gave {} as path", contact.path);
         println!("you gave {} as operation", operation);
        panic!("possible operations:
            -a: add a number
            -f: fetch a number
            ");
    }
}

struct Contact {
    name: String,
    number: String,
    path: String,
}
impl Contact {
    fn build(args: &[String]) -> Self { //nous allons decomposer les args. why does the type have to be &[String] instead of vec<String> ?
        let name = args[1].to_string().to_lowercase();
        let number = args[2].to_string();
        let path = args[3].to_string();
        Contact { name, number, path }
    }
}

struct Person {
    name: String,
    path: String,
}
impl Person {
    fn build(args: &[String]) -> Self {
        let name = args[1].to_string().to_lowercase();
        let path = args[2].to_string();
        Person { name, path }
    }
}

fn write_contact(mut file: File, name: String, number: String) {
    let person = format!("{name} : {number} \n");
    match file.write_all(person.as_bytes()) {
        Ok(_) => {
            println!("success write ");
        }
        Err(e) => {
            println!("couldnt write: {}", e);
        }
    }
}

fn add_contact(contact: Contact) {
    let file = OpenOptions::new().append(true).open(&contact.path);
    match file{
        Ok(file) => {
            write_contact(file, contact.name, contact.number);

        }
        Err(..) => {
            let create = File::create(&contact.path);
            match create {
                Ok(mut file) => {
                    let _ = file.write_all(String::from("Contact List: \n").as_bytes()); //how to write on one line
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
fn fetch_contact(name:String, path: String){
    let open = File::open(&path);
    let mut reader = BufReader::new(open.unwrap());
    let mut line = String::new();
    let mut matched = false;
    loop {
        line.clear();
    match reader.read_line(&mut line) { //read line va retourner le byte lus. mais line contiendras le texte dans la ligne
        Ok(byte_read) => {
            if line.contains(&name) { //and then we check with the .containts() trait
                println!("{line}");
                matched = true;
            }
            else if byte_read == 0 && matched == false {
                println!("{} does not containt a contact for {}", path, name);
                break;
            }
            else if matched == true && byte_read == 0{
                break;
            }

        }
        Err(e) => {
            panic!("could not open file: {}", e);
        }
    }
    }
}
