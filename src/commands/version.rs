use colored::Colorize;
use weather::{REPO_LINK, VERSION};

pub fn version() {
   let version = format!("v{}", VERSION).bright_blue();
   println!("{} {}", "Weather ".bright_purple().bold(), version);
   println!(
      "Check the latest version at {}\n",
      REPO_LINK.bright_blue().underline()
   )
}
