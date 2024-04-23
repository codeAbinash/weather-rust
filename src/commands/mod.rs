pub mod help;
pub mod invalid;
pub mod version;

pub fn execute_command(command: &str) {
   match command {
      "--version" => version::version(),
      "-v" => version::version(),
      "--help" => help::help(),
      "-h" => help::help(),
      _ => invalid::invalid(),
   }
}
