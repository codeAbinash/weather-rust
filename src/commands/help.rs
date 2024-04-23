use colored::*;
use weather::{REPO_LINK, VERSION};

pub fn help() {
   println!("{} {}", "Weather".bright_purple(), VERSION.bright_blue());
   println!();
   println!("USAGE: {} [OPTIONS]", "Weather");
   println!("");
   println!("OPTIONS:");
   println!("\t--version\tPrints version information");
   println!("\t--help\t\tPrints help information");
   println!("");
   println!("EXAMPLES:");
   println!("\tweather\t\t\tPrints the current weather report");
   println!("\tweather --version\tPrints the version information");
   println!("\tweather --help\t\tPrints the help information");
   println!();
   println!("Read More: {}\n", REPO_LINK.bright_blue().underline());
   println!();
}
