use rustyline::{error::ReadlineError, history::DefaultHistory, Editor};

use crate::{command::Command, terminal::default_terminal::DefaultTerminal};

pub struct Shell {
    terminal: DefaultTerminal,
    // rl: rustyline::Editor<&'a DefaultTerminal, DefaultHistory>,
}

impl Shell {
    pub fn builder() -> ShellBuilder {
        ShellBuilder::default()
    }

    pub async fn start(&self) {
        let mut rl: Editor<&DefaultTerminal, rustyline::history::FileHistory> =
            rustyline::Editor::<&DefaultTerminal, DefaultHistory>::new().unwrap();

        let history_path = self.start_history_log(&mut rl);

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    match rl.add_history_entry(line.as_str()) {
                        Ok(_) => {}
                        Err(_) => log::warn!("Cannot add command entry to history"),
                    }

                    let output = self.terminal.run_command(line).await;
                    print!("\t{}", String::from_utf8(output).unwrap());
                }
                Err(ReadlineError::Interrupted) => {
                    println!("Bye");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("Bye");
                    break;
                }
                Err(err) => {
                    println!("Error: {:?}", err);
                    break;
                }
            }
        }

        rl.save_history(&history_path).unwrap();
    }

    fn start_history_log<'a>(
        &'a self,
        rl: &mut Editor<&'a DefaultTerminal, rustyline::history::FileHistory>,
    ) -> std::path::PathBuf {
        let xdg_dirs = xdg::BaseDirectories::with_prefix("wswitch_interface")
            .expect("Cannot access XDG environment");

        let history_path = xdg_dirs
            .place_state_file("history.txt")
            .expect("Cannot create history file");

        log::info!("{history_path:?}");

        log::debug!(
            "Using history located at {}",
            history_path.to_str().unwrap()
        );

        let _ = rl.load_history(&history_path);
        let _ = rl.set_helper(Some(&self.terminal));

        history_path
    }
}

#[derive(Default)]
pub struct ShellBuilder {
    commands: Vec<Box<dyn Command>>,
    terminal: DefaultTerminal,
    // history_path: String,
}

impl ShellBuilder {
    pub fn new() -> ShellBuilder {
        ShellBuilder {
            commands: Default::default(),
            terminal: Default::default(),
        }
    }

    pub fn commands(mut self, commands: Vec<Box<dyn Command>>) -> ShellBuilder {
        self.commands = commands;
        self
    }

    pub fn build(mut self) -> Shell {
        self.terminal = DefaultTerminal::builder().commands(self.commands).build();

        Shell {
            terminal: self.terminal,
        }
    }
}
