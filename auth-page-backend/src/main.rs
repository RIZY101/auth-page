#[macro_use] extern crate rocket;
use rocket::response::content;
use rocket::response::Redirect;
use rand::Rng;
use rand::seq::SliceRandom;
use rocket::form::{Form, Strict};
use std::fs::File;
use std::io::{self, BufRead, BufWriter, Write};
use std::path::Path;

const MNEMONICS: &'static [&'static str] = &["Please Excuse My Dear Aunt Sally 12", "Eggs Are Deliciously Good Breakfast Energy 34", "Fat Alley Cats Eat Alot Of Garbage 56", "All Cows Eat Lots Of Green Grass 78", "Goblins Bring Death For All Creatures 91"];
const MNEMONICS_PASS: &'static [&'static str] = &["PEMDAS12", "EADGBE34", "FACEAOG45", "ACELOGG78", "GBDFAC91"];

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

fn contains_at(new_user: &Form<User>) -> bool {
    if new_user.email.contains("@") {
        true
    } else {
        false
    }
}

fn atleast_8(new_user: &Form<User>) -> bool {
    if new_user.password.chars().count() >= 8 {
        true
    } else {
        false
    }
}

fn contains_two_nums(new_user: &Form<User>) -> bool {
    let mut count = 0;
    for c in new_user.password.chars() {
        if c.is_numeric() {
            count += 1;
        }
    }
    if count >= 2 {
        true
    } else {
        false
    }
}

fn mnemoic_in_list(new_user: &Form<User>) -> bool {
    if MNEMONICS_PASS.contains(&new_user.password.as_str()) {
        true
    } else {
        false
    }
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
fn index() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
    <!doctype html>
    <html lang="en">
    
      <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>Auth Experiment</title>
        <meta name="description" content="Auth Experiment">
        <!-- Pico.css -->
        <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
      </head>
    
      <body>
        <!-- Nav -->
        <nav class="container-fluid">
        </nav><!-- ./ Nav -->
    
        <!-- Main -->
        <main class="container">
          <article class="grid">
            <div>
              <hgroup>
                <h1>Authentication Experiment</h1>
                <h2>Insrtuctions</h2>
              </hgroup>
              <form action="/start" method="GET"> 
                <p>By using this website you are agreeing to taking part in a controlled experiment about web service authentication. In this experiment you will be given one of two authentication methods to use. A standard password, or a mnemoic. You will recieve up to $4 for completing the experiment. $1 for creating an account with your assinged method, $1 for logging in after creating an account, $1 for logging in a second time 24 hours from your first login attempt, and $1 for completing the short exit survey. Please note if you can not remember your password you can use the forgot my pasword button on the login page in order to login (doing this will still count towards your $1 for that task). You will recieve an email reminding you to login after 24 hours has passed, and one asking what your prefered payment method is after completing the experiment. Please note all traffic is encrypted over https and industry leading password hashing (Argon2) and protection techinques so your data is safe and secure from malicous actors. Click the button bellow to get started.</p>
                <button type="submit" class="contrast">Start</button>
              </form>
            </div>
          </article>
        </main><!-- ./ Main -->
        
        <!-- Footer -->
        <footer class="container-fluid">
          <small>Built using  <a href="https://picocss.com" class="secondary">Pico CSS</a>
        </footer><!-- ./ Footer -->
      </body>
    </html>    
    "#)
}

#[get("/start")]
fn start() -> Redirect {
    let mut rng = rand::thread_rng();
    let n1: u8 = rng.gen();
    if n1 % 2 == 0 {
        Redirect::to("/password")
    } else {
        Redirect::to("/mnemonic")
    }
}

#[get("/password")]
fn password() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
    <!doctype html>
    <html lang="en">
    
      <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>Auth Experiment</title>
        <meta name="description" content="Auth Experiment">
        <!-- Pico.css -->
        <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css"> 
      </head>
    
      <body>
        <!-- Nav -->
        <nav class="container-fluid">
        </nav><!-- ./ Nav -->
    
        <!-- Main -->
        <main class="container">
          <article class="grid">
            <div>
              <hgroup>
                <h1>Create Account</h1>
                <h2>Standard Password</h2>
              </hgroup>
              <form action="/create/password" method="POST">
                <input type="text" name="email" placeholder="Email" aria-label="Email" autocomplete="nickname" required>
                <input type="" name="password" placeholder="Password" aria-label="Password" autocomplete="current-password" required>
                <button type="submit" class="contrast">Create</button>
              </form>
            </div>
            <div>
            <hgroup>
              <h1>Rules</h1>
              <h2>Please ensure that your password meets the length complexity of...</h2>
            </hgroup>
            </div>
          </article>
        </main><!-- ./ Main -->
    
        <!-- Footer -->
        <footer class="container-fluid">
          <small>Built using  <a href="https://picocss.com" class="secondary">Pico CSS</a>
        </footer><!-- ./ Footer -->
      </body>
    </html>
    "#)
}

#[get("/mnemonic")]
fn mnemonic() -> content::RawHtml<&'static str> {
    content::RawHtml(r#"
    <!doctype html>
    <html lang="en">
    
      <head>
        <meta charset="utf-8">
        <meta name="viewport" content="width=device-width, initial-scale=1">
        <title>Auth Experiment</title>
        <meta name="description" content="Auth Experiment">
        <!-- Pico.css -->
        <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
      </head>
    
      <body>
        <!-- Nav -->
        <nav class="container-fluid">
        </nav><!-- ./ Nav -->
    
        <!-- Main -->
        <main class="container">
          <article class="grid">
            <div>
              <hgroup>
                <h1>Create Account</h1>
                <h2>Mnemonic Passphrase</h2>
              </hgroup>
              <form action="/create/mnemonic" method="POST">
                <input type="text" name="email" placeholder="Email" aria-label="Email" autocomplete="nickname" required>
                <input type="" name="password" placeholder="Password" aria-label="Password" autocomplete="current-password" required>
                <button type="submit" class="contrast">Create</button>
              </form>
            </div>
            <div>
            <hgroup>
              <h1>Rules</h1>
              <h2>Please use the generator to choose a mnemonic. Then type the first letter of each word in the phrase into the password box...</h2>
            </hgroup>
            </div>
          </article>
        </main><!-- ./ Main -->
    
        <!-- Footer -->
        <footer class="container-fluid">
          <small>Built using  <a href="https://picocss.com" class="secondary">Pico CSS</a>
        </footer><!-- ./ Footer -->
      </body>
    </html>
    "#)
}

#[get("/get/mnemonic")]
fn get_mnemonic_route() -> String {
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
fn new_password_user(new_user: Form<User>) -> content::RawHtml<&'static str> {
    if contains_at(&new_user) {
        if atleast_8(&new_user) && contains_two_nums(&new_user) {
            println!("{} and {}", new_user.email, new_user.password);
            content::RawHtml(r#"
            <!doctype html>
            <html lang="en">
            
              <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Auth Experiment</title>
                <meta name="description" content="Auth Experiment">
                <!-- Pico.css -->
                <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
              </head>
            
              <body>
                <!-- Nav -->
                <nav class="container-fluid">
                </nav><!-- ./ Nav -->
            
                <!-- Main -->
                <main class="container">
                  <article class="grid">
                    <div>
                      <hgroup>
                        <h1>Authentication Experiment</h1>
                        <h2>Account Created</h2>
                      </hgroup>
                      <form action="/login" method="GET"> 
                        <p>At this point you have successfully created your account and earned $1. Please login to continue the process, and to earn another $1!</p>
                        <button type="submit" class="contrast">Login</button>
                      </form>
                    </div>
                  </article>
                </main><!-- ./ Main -->
                
                <!-- Footer -->
                <footer class="container-fluid">
                  <small>Built using  <a href="https://picocss.com" class="secondary">Pico CSS</a>
                </footer><!-- ./ Footer -->
              </body>
            </html>
            "#)
        } else {
            content::RawHtml(r#"
            <!doctype html>
            <html lang="en">
            
              <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Auth Experiment</title>
                <meta name="description" content="Auth Experiment">
                <!-- Pico.css -->
                <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
              </head>
            
              <body>
                <!-- Nav -->
                <nav class="container-fluid">
                </nav><!-- ./ Nav -->
            
                <!-- Main -->
                <main class="container">
                  <article class="grid">
                    <div>
                      <hgroup>
                        <h1>Authentication Experiment</h1>
                        <h2>Error Account Not Created</h2>
                      </hgroup>
                      <form action="/password" method="GET"> 
                        <p>Password did not contain two numbers or it was not atleast 8 characters. Please use the back button and try again.</p>
                        <button type="submit" class="contrast">Back</button>
                      </form>
                    </div>
                  </article>
                </main><!-- ./ Main -->
                
                <!-- Footer -->
                <footer class="container-fluid">
                  <small>Built using  <a href="https://picocss.com" class="secondary">Pico CSS</a>
                </footer><!-- ./ Footer -->
              </body>
            </html>
            "#)
        }
    } else {
        content::RawHtml(r#"
        <!doctype html>
        <html lang="en">
        
          <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>Auth Experiment</title>
            <meta name="description" content="Auth Experiment">
            <!-- Pico.css -->
            <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
          </head>
        
          <body>
            <!-- Nav -->
            <nav class="container-fluid">
            </nav><!-- ./ Nav -->
        
            <!-- Main -->
            <main class="container">
              <article class="grid">
                <div>
                  <hgroup>
                    <h1>Authentication Experiment</h1>
                    <h2>Error Account Not Created</h2>
                  </hgroup>
                  <form action="/password" method="GET"> 
                    <p>Email provided did not appear to be valid. Please use the back button and try again.</p>
                    <button type="submit" class="contrast">Back</button>
                  </form>
                </div>
              </article>
            </main><!-- ./ Main -->
            
            <!-- Footer -->
            <footer class="container-fluid">
              <small>Built using  <a href="https://picocss.com" class="secondary">Pico CSS</a>
            </footer><!-- ./ Footer -->
          </body>
        </html>
        "#)
    }
}

#[post("/create/mnemonic", data = "<new_user>")]
fn new_mnemonic_user(new_user: Form<User>) -> content::RawHtml<&'static str> {
    if contains_at(&new_user) {
        if mnemoic_in_list(&new_user) {
            println!("{} and {}", new_user.email, new_user.password);
            content::RawHtml(r#"
            <!doctype html>
            <html lang="en">
            
              <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Auth Experiment</title>
                <meta name="description" content="Auth Experiment">
                <!-- Pico.css -->
                <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
              </head>
            
              <body>
                <!-- Nav -->
                <nav class="container-fluid">
                </nav><!-- ./ Nav -->
            
                <!-- Main -->
                <main class="container">
                  <article class="grid">
                    <div>
                      <hgroup>
                        <h1>Authentication Experiment</h1>
                        <h2>Account Created</h2>
                      </hgroup>
                      <form action="/login" method="GET"> 
                        <p>At this point you have successfully created your account and earned $1. Please login to continue the process, and to earn another $1!</p>
                        <button type="submit" class="contrast">Login</button>
                      </form>
                    </div>
                  </article>
                </main><!-- ./ Main -->
                
                <!-- Footer -->
                <footer class="container-fluid">
                  <small>Built using  <a href="https://picocss.com" class="secondary">Pico CSS</a>
                </footer><!-- ./ Footer -->
              </body>
            </html>
            "#)
        } else {
            content::RawHtml(r#"
            <!doctype html>
            <html lang="en">
            
              <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>Auth Experiment</title>
                <meta name="description" content="Auth Experiment">
                <!-- Pico.css -->
                <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
              </head>
            
              <body>
                <!-- Nav -->
                <nav class="container-fluid">
                </nav><!-- ./ Nav -->
            
                <!-- Main -->
                <main class="container">
                  <article class="grid">
                    <div>
                      <hgroup>
                        <h1>Authentication Experiment</h1>
                        <h2>Error Account Not Created</h2>
                      </hgroup>
                      <form action="/mnemonic" method="GET"> 
                        <p>Mnemonic not a part of the approved list. Please use the back button and try again.</p>
                        <button type="submit" class="contrast">Back</button>
                      </form>
                    </div>
                  </article>
                </main><!-- ./ Main -->
                
                <!-- Footer -->
                <footer class="container-fluid">
                  <small>Built using  <a href="https://picocss.com" class="secondary">Pico CSS</a>
                </footer><!-- ./ Footer -->
              </body>
            </html>
            "#)   
        }
    } else {
        content::RawHtml(r#"
        <!doctype html>
        <html lang="en">
        
          <head>
            <meta charset="utf-8">
            <meta name="viewport" content="width=device-width, initial-scale=1">
            <title>Auth Experiment</title>
            <meta name="description" content="Auth Experiment">
            <!-- Pico.css -->
            <link rel="stylesheet" href="https://unpkg.com/@picocss/pico@latest/css/pico.min.css">
          </head>
        
          <body>
            <!-- Nav -->
            <nav class="container-fluid">
            </nav><!-- ./ Nav -->
        
            <!-- Main -->
            <main class="container">
              <article class="grid">
                <div>
                  <hgroup>
                    <h1>Authentication Experiment</h1>
                    <h2>Error Account Not Created</h2>
                  </hgroup>
                  <form action="/mnemonic" method="GET"> 
                    <p>Email provided did not appear to be valid. Please use the back button and try again.</p>
                    <button type="submit" class="contrast">Back</button>
                  </form>
                </div>
              </article>
            </main><!-- ./ Main -->
            
            <!-- Footer -->
            <footer class="container-fluid">
              <small>Built using  <a href="https://picocss.com" class="secondary">Pico CSS</a>
            </footer><!-- ./ Footer -->
          </body>
        </html>
        "#)
    }
}

#[launch]
fn rocket() -> _ {
    setup_db();
    rocket::build().mount("/", routes![index, mnemonic, password, get_mnemonic_route, new_password_user, new_mnemonic_user, start])
}
