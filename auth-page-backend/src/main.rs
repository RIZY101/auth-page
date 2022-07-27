#[macro_use] extern crate rocket;
use rocket::response::content;
use rocket::response::Redirect;
use rand::Rng;
use rocket::form::{Form};
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::path::Path;
use chrono;

const MNEMONICS: &'static [&'static str] = &["Please Excuse My Dear Aunt Sally 12", "Eggs Are Deliciously Good Breakfast Energy 34", "Fat Alley Cats Eat Alot Of Garbage 56", "All Cows Eat Lots Of Green Grass 78", "Goblins Bring Death For All Creatures 91"];
const MNEMONICS_PASS: &'static [&'static str] = &["PEMDAS12", "EADGBE34", "FACEAOG56", "ACELOGG78", "GBDFAC91"];

//enum DbContent {
//    Password,
//    Date,
//    Time,
//    TimeCode,
//    LoginAttempts,
//    ForgotPasswordUses,
//    LoginSuccesses
//}

#[derive(FromForm)]
struct User {
    email: String,
    password: String
}

#[derive(FromForm)]
struct Forgot {
    email: String
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

fn mnemoic_in_list_str(string: &str) -> bool {
  if MNEMONICS_PASS.contains(&string) {
      true
  } else {
      false
  }
}

fn increment(str: String) -> String {
    let mut integer = str.parse::<i32>().unwrap();
    integer += 1;
    integer.to_string()
}

fn get_mnemoic_index_from_str(string: &str) -> usize {
  match string {
    "PEMDAS12"  => 0,
    "EADGBE34"  => 1, 
    "FACEAOG56" => 2, 
    "ACELOGG78" => 3, 
    "GBDFAC91"  => 4,
    _           => 0
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

fn create_db(new_user: &Form<User>) {
    let path = String::from(format!("target/{}.db", new_user.email));
    let db_contents = String::from(format!("{} {} 0 0 0", new_user.password, chrono::offset::Utc::now()));
    let mut file = File::create(path).unwrap();
    file.write(db_contents.as_bytes()).unwrap();
}

fn read_db(user: &Form<User>) -> Vec<String> {
    let path = String::from(format!("target/{}.db", user.email));
    let mut lines_vec = vec![];
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(one_line) = line {
                lines_vec.push(one_line.to_string());
            }
        }
    }
    return lines_vec
}

//No traditional function overloading in rust :(
fn read_db2(user: &Form<Forgot>) -> Vec<String> {
  let path = String::from(format!("target/{}.db", user.email));
  let mut lines_vec = vec![];
  if let Ok(lines) = read_lines(path) {
      for line in lines {
          if let Ok(one_line) = line {
              lines_vec.push(one_line.to_string());
          }
      }
  }
  return lines_vec
}

fn write_db(line: String, user: &Form<User>) {
    let path = String::from(format!("target/{}.db", user.email));
    let mut file = File::create(path).unwrap();
    file.write(line.as_bytes()).unwrap();

}

//No traditional function overloading in rust :(
fn write_db2(line: String, user: &Form<Forgot>) {
  let path = String::from(format!("target/{}.db", user.email));
  let mut file = File::create(path).unwrap();
  file.write(line.as_bytes()).unwrap();

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
                <p>By using this website you are agreeing to taking part in a controlled experiment about web service authentication. In this experiment you will be given one of two authentication methods to use, a standard password or a mnemonic (more about this method will be explained if you get selected for it). 
                You will recieve up to $4 for completing the whole experiment. $1 for creating an account with your assinged method, $1 for logging in after creating an account, $1 for logging in a second time 24 hours from your first successful login, and $1 for completing the short exit survey. 
                Please note if you can not remember your password you can use the forgot my pasword button in order to retrieve your password and then achieve a successful login. 
                Also you will recieve an email reminding you to login after 24 hours has passed, and one reminding you to take the exit survey which will also ask what your prefered payment method is after completing the experiment (2 successful logins). 
                Lastly please note all traffic is encrypted over https, but we will be statistically and heuristically evaluating the passwords you provide. Once these statistics and results have been finished your password data will be deleted. 
                If you have any poroblems, questions, or concerns please email rizins@berkeley.edu <br> <br>
                Click the button bellow to get started.</p>
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

#[get("/login")]
fn login() -> content::RawHtml<&'static str> {
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
                <h1>Login</h1>
                <h2>Use Password/Mnemonic</h2>
              </hgroup>
              <form action="/login/verify" method="POST">
                <input type="text" name="email" placeholder="Email" aria-label="Email" autocomplete="nickname" required>
                <input type="" name="password" placeholder="Password" aria-label="Password" autocomplete="current-password" required>
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
              <h2>Please ensure that your password is atleast 8 characters and conatins 2 numbers.</h2>
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
              <p>A mnemonic is a pattern of letters, ideas, or associations that help you remember something. In this case you can choose one of the five mnemonics bellow to use as your password, and hopefully the silly sentence 
              will help you remember the password it is associated with. Please note you must use one of the passwords associated with the mnemonic sentence located after the "->", and yes it must be entered with all capital letters.</p>
              <p>"Please Excuse My Dear Aunt Sally 12" -> PEMDAS12 <br>
              "Eggs Are Deliciously Good Breakfast Energy 34" -> EADGBE34 <br>
              "Fat Alley Cats Eat Alot Of Garbage 56" -> FACEAOG56 <br>
              "All Cows Eat Lots Of Green Grass 78" -> ACELOGG78 <br>
              "Goblins Bring Death For All Creatures 91" -> GBDFAC91 </p>
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

#[post("/create/password", data = "<new_user>")]
fn new_password_user(new_user: Form<User>) -> content::RawHtml<&'static str> {
    if contains_at(&new_user) {
        if atleast_8(&new_user) && contains_two_nums(&new_user) {
            create_db(&new_user);
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
                        <p>Password did not contain two numbers, or it was not atleast 8 characters. Please use the back button and try again.</p>
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
            create_db(&new_user);
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

#[get("/forgot")]
fn forgot() -> content::RawHtml<&'static str> {
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
              <h1>Recover Account</h1>
              <h2>Get Password/Mnemoic</h2>
            </hgroup>
            <form action="/forgot/password" method="POST">
              <p>Enter your email to recieve your password/mnemonic used to login.</p>
              <input type="text" name="email" placeholder="Email" aria-label="Email" autocomplete="nickname" required>
              <button type="submit" class="contrast">Retrieve</button>
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

#[post("/forgot/password",  data = "<user>")]
fn forgot_pass(user: Form<Forgot>) -> content::RawHtml<String> {
  let lines_vec = read_db2(&user);
  if lines_vec.len() != 0 {
    let mut split: Vec<String> = lines_vec[0].split_whitespace().map(str::to_string).collect();
    if mnemoic_in_list_str(split[0].as_str()) {
      let my_str = &split[5];
      split[5] = increment(my_str.to_string());
      let string_to_write = String::from(format!("{} {} {} {} {} {} {}", split[0], split[1], split[2], split[3], split[4], split[5], split[6]));
      write_db2(string_to_write, &user);
      let html = String::from(format!(r#"
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
                  <h1>Recover Account</h1>
                  <h2>Get Password/Mnemoic</h2>
                </hgroup>
                <form action="/login" method="GET">
                  <p>Your password is "{}" -> {}</p>
                  <button type="submit" class="contrast">Back To Login</button>
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
      "#, split[0], MNEMONICS[get_mnemoic_index_from_str(split[0].as_str())]));
      content::RawHtml(html)
    } else {
      let my_str = &split[5];
      split[5] = increment(my_str.to_string());
      let string_to_write = String::from(format!("{} {} {} {} {} {} {}", split[0], split[1], split[2], split[3], split[4], split[5], split[6]));
      write_db2(string_to_write, &user);
      let html = String::from(format!(r#"
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
                  <h1>Recover Account</h1>
                  <h2>Get Password/Mnemoic</h2>
                </hgroup>
                <form action="/login" method="GET">
                  <p>Your password is {}</p>
                  <button type="submit" class="contrast">Back To Login</button>
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
      "#, split[0]));
      content::RawHtml(html)
    }
  } else {
    let html = String::from(r#"
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
                <h1>Recover Account</h1>
                <h2>Error Password/Mnemoic Not Found</h2>
              </hgroup>
              <form action="/forgot" method="GET">
                <p>Your password could not be found. If you think this was in error try hitting the back button and try to enter your email again.</p>
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
    "#);
    content::RawHtml(html)
  }
}

#[post("/login/verify", data = "<user>")]
fn login_verify(user: Form<User>) -> content::RawHtml<&'static str> {
    if contains_at(&user) {
        let lines_vec = read_db(&user);
        let mut split: Vec<String> = lines_vec[0].split_whitespace().map(str::to_string).collect();
        if split[0] == user.password {
          if split[6] == "2" {
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
                        <h2>Error Login Not Needed</h2>
                      </hgroup>
                      <form action="/login" method="GET"> 
                        <p>This account has already logged in successfully twice, and has earned the full reward. If you think you got to this page by mistake please use the back button to login with your account. Otherwise please take the exit survey located at this <a href="https://docs.google.com/forms/d/e/1FAIpQLSdwg_2f8VTTEXKVzi1As2gGk9NrUgxCx4gRntdE514YS8N5gQ/viewform">link</a>.</p>
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
          } else {
            //TODO add if to do the 24 hours stuff
            let my_str = &split[6];
            split[6] = increment(my_str.to_string());
            let my_str2 = &split[4];
            split[4] = increment(my_str2.to_string());
            let string_to_write = String::from(format!("{} {} {} {} {} {} {}", split[0], split[1], split[2], split[3], split[4], split[5], split[6]));
            write_db(string_to_write, &user);
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
                        <h2>Login Successful</h2>
                      </hgroup>
                        <p>At this point you have successfully logged into your account and earned another $1. If you have not logged in twice already please return to the login page in 24 hours and log in again. Please note you will also recieve an email reminder after it has been 24 hours, and hence when to return to the login page.
                        If this is your second successful login please complete the exit survey located at this <a href="https://docs.google.com/forms/d/e/1FAIpQLSdwg_2f8VTTEXKVzi1As2gGk9NrUgxCx4gRntdE514YS8N5gQ/viewform">link</a>.</p>
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
          let my_str = &split[4];
          split[4] = increment(my_str.to_string());
          let string_to_write = String::from(format!("{} {} {} {} {} {} {}", split[0], split[1], split[2], split[3], split[4], split[5], split[6]));
          write_db(string_to_write, &user);
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
                        <h2>Error Login Not Successful</h2>
                      </hgroup>
                      <form action="/login" method="GET"> 
                        <p>Password provided did not appear to be valid. Please use the back button and try again.</p>
                        <button type="submit" class="contrast">Back</button>
                      </form>
                      <form action="/forgot" method="GET"> 
                      <p>Or use the forgot my password button to obtain your password.</p>
                      <button type="submit" class="contrast">Forgot Password</button>
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
                    <h2>Error Login Not Successful</h2>
                  </hgroup>
                  <form action="/login" method="GET"> 
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
    rocket::build().mount("/", routes![index, mnemonic, password, new_password_user, new_mnemonic_user, start, login, login_verify, forgot, forgot_pass])
}
