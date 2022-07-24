#[macro_use] extern crate rocket;
use rand::seq::SliceRandom;
use rocket::form::{Form, Strict};

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

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/mnemonic")]
fn mnemonic() -> String {
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
    rocket::build().mount("/", routes![index, mnemonic, new_password_user, new_mnemonic_user])
}
