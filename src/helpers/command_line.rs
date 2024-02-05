use crossterm::{
    style::{Color, ResetColor, SetForegroundColor},
    ExecutableCommand,
};
use std::io::{stdin, stdout};

#[derive(PartialEq, Debug)]
pub enum PrintCommand {
    Primary,
    Secondary,
    Issue,
}

impl PrintCommand {
    //Print messages with different colors depends on the type
    pub fn print_msg(&self, position: &str, statement: &str) {
        let mut stdout: std::io::Stdout = stdout();

        let statement_color = match self {
            Self::Primary => Color::Cyan,
            Self::Secondary => Color::Yellow,
            Self::Issue => Color::Magenta,
        };

        stdout.execute(SetForegroundColor(Color::Blue)).unwrap();
        println!("Agent: {}", position);

        stdout.execute(SetForegroundColor(statement_color)).unwrap();
        println!("Statement: {}: ", statement);

        stdout.execute(ResetColor).unwrap();
    }
}

//Get response from the user for any
//questions and return back a string with answers
pub fn get_response(questions: &str) -> String {
    let mut stdout: std::io::Stdout = stdout();

    stdout.execute(SetForegroundColor(Color::Green)).unwrap();
    println!("");
    println!("{}", questions);
    stdout.execute(ResetColor).unwrap();

    let mut user_input: String = String::new();

    stdin()
        .read_line(&mut user_input)
        .expect("Failed to read user input.");

    stdout.execute(ResetColor).unwrap();

    user_input.trim().to_string()
}

//ask th user whether to proceed with creating the project
pub fn decision_to_proceed() -> bool {
    let mut stdout: std::io::Stdout = stdout();

    loop {
        stdout.execute(SetForegroundColor(Color::DarkCyan)).unwrap();
        println!("Confirm if you want to continue.");
        stdout.execute(ResetColor).unwrap();

        stdout.execute(SetForegroundColor(Color::Green)).unwrap();
        println!("[1] Continue.");
        stdout.execute(SetForegroundColor(Color::Red)).unwrap();
        println!("[2] Stop.");
        stdout.execute(ResetColor).unwrap();

        let mut response: String = String::new();

        stdin()
            .read_line(&mut response)
            .expect("Failed to read the response.");

        let response: String = response.trim().to_lowercase();

        match response.as_str() {
            "1" | "ok" | "continue" => return true,
            "2" | "no" | "stop" => return false,
            _ => {
                stdout
                    .execute(SetForegroundColor(Color::DarkMagenta))
                    .unwrap();
                println!("Invalid response.");
                stdout.execute(ResetColor).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primary_msg_test() {
        PrintCommand::Primary.print_msg("Manager", "This is a test statement for primary.");
    }

    #[test]
    fn secondary_msg_test() {
        PrintCommand::Secondary.print_msg("Manager", "This is a test statement for secondary.");
    }

    #[test]
    fn Issue_msg_test() {
        PrintCommand::Issue.print_msg("Manager", "This is a test statement for issue.");
    }
}
