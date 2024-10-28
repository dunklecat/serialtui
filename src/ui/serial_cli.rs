use rustyline::{error::ReadlineError, history::DefaultHistory, Editor};

use crate::{command::command_manager::CommandManager, command::Command};

pub struct SerialCli {
    command_manager: CommandManager,
    // rl: rustyline::Editor<&'a DefaultTerminal, DefaultHistory>,
}

impl SerialCli {
    pub fn builder() -> SerialCliBuilder {
        SerialCliBuilder::default()
    }

    pub async fn start(&self) {
        let mut rl: Editor<&CommandManager, rustyline::history::FileHistory> =
            rustyline::Editor::<&CommandManager, DefaultHistory>::new().unwrap();

        let history_path = self.start_history_log(&mut rl);

        loop {
            let readline = rl.readline(">> ");
            match readline {
                Ok(line) => {
                    match rl.add_history_entry(line.as_str()) {
                        Ok(_) => {}
                        Err(_) => log::warn!("Cannot add command entry to history"),
                    }

                    let output = self.command_manager.run_command(line).await;
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
        rl: &mut Editor<&'a CommandManager, rustyline::history::FileHistory>,
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
        let _ = rl.set_helper(Some(&self.command_manager));

        history_path
    }
}

#[derive(Default)]
pub struct SerialCliBuilder {
    commands: Vec<Box<dyn Command>>,
    command_manager: CommandManager,
    // history_path: String,
}

impl SerialCliBuilder {
    pub fn new() -> SerialCliBuilder {
        SerialCliBuilder {
            commands: Default::default(),
            command_manager: Default::default(),
        }
    }

    pub fn commands(mut self, commands: Vec<Box<dyn Command>>) -> SerialCliBuilder {
        self.commands = commands;
        self
    }

    pub fn build(mut self) -> SerialCli {
        self.command_manager = CommandManager::builder().commands(self.commands).build();

        SerialCli {
            command_manager: self.command_manager,
        }
    }
}
