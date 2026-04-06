


// Ascii art logo!!!
// raw string: r#""#
const LOGO: &str = r#"
============================================================
      __               ____                              
     / /  ___  ___ _  / __/_ ____ _  __ _  ___ _______ __
    / /__/ _ \/ _ `/ _\ \/ // /  ' \/  ' \/ _ `/ __/ // /
   /____/\___/\_, / /___/\_,_/_/_/_/_/_/_/\_,_/_/  \_, / 
             /___/                                /___/

============================================================
"#;
// const RULE: &str = "--------------------------------------------------------------------------------";
const DESC: &str = "         A simple CLI for sumarising server logs";
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Entry Point
fn main() {
    print_logo();
}

fn print_logo(){
    // \x1B[2J (or \x1B[J) clears the screen.
    // \x1B[1;1H (or \x1B[H) moves the cursor to the top-left corner.
    print!("{}[2J{}[1;1H", 27 as char, 27 as char);
    
    // macro: expands to interperate variable, print the line to stdout, then an additional \n
    println!("{}", LOGO);
    println!("{}", DESC);
    println!("---------------------------{}---------------------------", VERSION);
}