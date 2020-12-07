use crate::cli::command;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Cli;

const CMD_HISTORY_PATH: &str = "command_history.txt";

impl Cli {
    pub fn start() {
        let mut editor = Editor::<()>::new();
        editor
            .load_history(CMD_HISTORY_PATH)
            .unwrap_or_else(|e| println!("No previous history {}", e));

        let (_, cmd_alias) = command::get_commands();

        loop {
            let res_line = editor.readline(">>");
            match res_line {
                Ok(line) => {
                    let params = command::parse(&line);
                    if params.is_empty() {
                        continue;
                    }

                    // 第一个参数为命令别名
                    match cmd_alias.get(&params[0]) {
                        Some(cmd) => {
                            cmd.execute(&params);
                        }
                        None => match params[0] {
                            "" => continue,
                            "quit" | "q!" => break,
                            "help" | "h" => println!("Print help command"),
                            x => println!("Unknown command: {}", x),
                        },
                    }

                    editor.add_history_entry(line.as_str());
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        editor.save_history(CMD_HISTORY_PATH).unwrap();
    }
}
