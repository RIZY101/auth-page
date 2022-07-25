#[macro_use] extern crate rocket;
use rand::seq::SliceRandom;
use rocket::form::{Form, Strict};
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

const MNEMONICS: &'static [&'static str] = &["Please Excuse My Dear Aunt Sally", "Eggs Are Deliciously Good Breakfast Energy", "Fat Alley Cats Eat Alot Of Garbage", "All Cows Eat Lots Of Green Grass", "Goblins Bring Death For All Creatures"];

#[derive(FromForm)]
struct User {
    email: String,
    password: String
}

fn get_mnemonic() -> String {
    MNEMONICS
    .choose(&mut rand::thread_rng())
    .unwrap()
    .to_string()
}

fn get_password_from_mnemonic(mnemonic: &String) -> String {
    let initials: String = mnemonic
    .split(" ")                     // create an iterator, yielding words
    .flat_map(|s| s.chars().nth(0)) // get the first char of each word
    .collect();                     // collect the result into a String
    return initials
}

//Function source code from: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn setup_db() {
    let _ = File::create("target/test.db");
}

fn read_db() -> Vec<String> {
    let mut lines_vec = vec![];
    if let Ok(lines) = read_lines("target/test.db") {
        for line in lines {
            if let Ok(one_line) = line {
                lines_vec.push(one_line.to_string());
            }
        }
    }
    return lines_vec
}

fn write_db(lines_vec: &mut Vec<String>) {
    let mut file = File::create("target/test.db").unwrap();
    for line in lines_vec {
        file.write(line.as_bytes()).unwrap();
    }   
}

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/mnemonic")]
fn mnemonic() -> String {
    //Just testing code (will remove later)
    let lines_vec = read_db();
    let mut lines_vec_new = vec! [];
    for ele in lines_vec {
        println!("{}", ele);
        lines_vec_new.push("New String per line that was there\n".to_string());
    }
    write_db(&mut lines_vec_new);
    //end of testing code 

    let rand_mnemonic = get_mnemonic();
    let pass = get_password_from_mnemonic(&rand_mnemonic);
    format!("Mnemonic: {}. So your password is {}!", rand_mnemonic, pass)
}

#[post("/create/password", data = "<new_user>")]
fn new_password_user(new_user: Form<User>) -> &'static str {
    "Password Account Created"
}

#[post("/create/mnemonic", data = "<new_user>")]
fn new_mnemonic_user(new_user: Form<User>) -> &'static str {
    "Mnemonic Account Created"
}

#[launch]
fn rocket() -> _ {
    setup_db();
    rocket::build().mount("/", routes![index, mnemonic, new_password_user, new_mnemonic_user])
}
