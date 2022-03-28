use std::io::{self, Write};
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub fn run(){
    let mut command = String::new();

    let mut rl = Editor::<()>::new();

    let default_prompt = "paradome> ";
    let shaping_prompt = ">>     ";
    
    let mut prompt: &str = default_prompt;

    loop{
        command.clear();
        // print!("{}",prompt);
        let readline = rl.readline(prompt);
        io::stdout().flush().unwrap();
        match readline{
            Ok(line) => {
                println!("{}",line);
                rl.add_history_entry(line.as_str());
                if line.trim().ends_with(",") || line.trim().ends_with(":") {
                    prompt = shaping_prompt;
                }else{
                    prompt = default_prompt;
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    }

}