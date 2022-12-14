use lsm_db::db::cmd::{print_help, Command};
use lsm_db::db::config::Config;
use lsm_db::db::db::Db;
use rustyline::error::ReadlineError;
use rustyline::Editor;

/// key-value server.
struct Server {
    /// key-value database.
    db: Db,
    /// readline editor.
    editor: Editor<()>,
    /// history file path.
    history_path: String,
}

impl Server {
    fn new(db: Db) -> Server {
        // set a cmd_history file to store history commands.
        let history_path = format!(
            "{}/.cmd_history",
            std::env::current_dir().unwrap().display().to_string()
        );
        // create an editor for reading lines.
        let mut editor = Editor::<()>::new().unwrap();
        // attempt to load history from file ./.cmd_history if it exists.
        let _ = editor.load_history(&history_path);
        Server {
            db: db,
            editor: editor,
            history_path: history_path,
        }
    }

    fn run(&mut self) {
        // print help options.
        print_help();
        // repeatedly read commands from the terminal and forward it to the db.
        loop {
            let cmd = self.get_next_cmd();
            match cmd {
                // terminate the server if typed in quit command.
                Command::Quit => break,
                // print help options if requested.
                Command::Help => print_help(),
                // forward other commands to the database.
                _ => self.db.handle_cmd(cmd),
            }
        }
    }

    fn get_next_cmd(&mut self) -> Command {
        loop {
            static PROMPT: &str = "(lsm_db) ";
            match self.editor.readline(PROMPT) {
                Ok(line) => {
                    // skip empty line.
                    if line.trim().len() == 0 {
                        continue;
                    }

                    // save the command as a history entry.
                    self.editor.add_history_entry(line.as_str());
                    if let Err(_) = self.editor.save_history(&self.history_path) {
                        log::warn!("Failed to save history file");
                    }

                    // split the command line into tokens.
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    // construct a commmand from tokens.
                    if let Some(cmd) = Command::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command");
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    // user pressed ctrl-c, prompt the user to type `quit` inorder to quit.
                    println!("Hint: type \"q\" or \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // user pressed ctrl-d, which is the equivalence of "quit" for our purposes
                    return Command::Quit;
                }
                Err(err) => {
                    log::error!("Unexpected error: {:?}", err);
                }
            }
        }
    }
}

fn main() {
    // create a db with the default config.
    let db = Db::new(Config::new());
    // create a server on which runs the db.
    Server::new(db).run();
}
