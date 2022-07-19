#[macro_use] extern crate rocket;
use rand::seq::SliceRandom;

const MNEMONICS: &'static [&'static str] = &["Please Excuse My Dear Aunt Sally", "Eggs Are Deliciously Good Breakfast Energy", "Fat Alley Cats Eat Alot Of Garbage", "All Cows Eat Lots Of Green Grass", "Goblins Bring Death For All Creatures"];

fn getMnemonic() -> String {
    MNEMONICS
    .choose(&mut rand::thread_rng())
    .unwrap()
    .to_string()
}

fn getPasswordFromMnemonic(mnemonic: &String) -> String {
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
    let randMnemonic = getMnemonic();
    let pass = getPasswordFromMnemonic(&randMnemonic);
    format!("Mnemonic: {}. So your password is {}!", randMnemonic, pass)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, mnemonic])
}
