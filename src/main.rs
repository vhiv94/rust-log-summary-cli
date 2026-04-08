#![doc = r"the main entry point of the program"]




#[doc = r"Ascii art logos are cool!"]
const LOGO: &str = r"
      __               ____                              
     / /  ___  ___ _  / __/_ ____ _  __ _  ___ _______ __
    / /__/ _ \/ _ `/ _\ \/ // /  ' \/  ' \/ _ `/ __/ // /
   /____/\___/\_, / /___/\_,_/_/_/_/_/_/_/\_,_/_/  \_, / 
             /___/                                /___/

"; 
const RULE: &str = "============================================================";
const DESC: &str = "         A simple CLI for sumarising server logs";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Entry Point
fn main() {
    reset_screen();
    print_logo();
    // hello_user();
    pick_a_number();
    // end_program();
}

#[doc = "uses control sequences to clear the screen, buffer, and reset the cursor"]
fn reset_screen() {
    print!("{0}[2J{0}[3J{0}[H", 27 as char);
}

#[doc = "print the ascii art logo"]
fn print_logo() {
    println!("{}", RULE);
    println!("{}", LOGO);
    println!("{}", RULE);
    println!("{}", DESC);
    println!(
        "---------------------------{}---------------------------",
        VERSION
    );
}

/* 
#[doc = "ask the user for their name, then greet them"]
fn hello_user() {
    println!("What's your name?");
    let mut name = String::new();
    match std::io::stdin()
    .read_line(&mut name) {
        Ok(_) => {
            reset_screen();
            println!("Hello, {}", name.trim())
        },   
        Err(e) => eprintln!("{}", e),
    }
}
 */

fn pick_a_number() {
    reset_screen();
    println!("Pick a number between 1 and 10:");
    let mut num= 2u32;
    let exp: u32;
    let mut numbuf = String::new();
    match std::io::stdin()
    .read_line( &mut numbuf) {
        Ok(_) => exp = numbuf.trim().parse().unwrap_or_default(),
        Err(e) => {
            eprintln!("{}", e);
            exp = 0;
        }
    }
    reset_screen();
    println!("{} times {} equals {}", num, exp, num * exp);
    num = num.pow(exp);
    println!("2 to the power of {} is {}", exp, num);
}

/* 
fn end_program() {
    println!();
    println!();
    println!("press ENTER to close...");

    let mut _quit = String::new();
    std::io::stdin().read_line(&mut _quit).expect("Failed to read line");
    reset_screen();
}
 */