use rustyline::hint::Hinter;
use rustyline::Context;
use rustyline_derive::{Completer, Helper, Highlighter, Validator};

use crate::{
    command::{
        command_manager::{CommandManager, CommandManagerBuilder},
        Command,
    },
    hint::default_hint::DefaultHint,
};

#[derive(Default, Completer, Helper, Highlighter, Validator)]
pub struct DefaultTerminal {
    command_manager: CommandManager,
}

impl DefaultTerminal {
    pub fn builder() -> DefaultTerminalBuilder {
        DefaultTerminalBuilder::default()
    }

    pub async fn run_command(&self, data: String) -> Vec<u8> {
        let mut tokens = data
            .split_whitespace()
            .map(|s| <String as std::str::FromStr>::from_str(s).unwrap());

        let cmd: Option<String> = tokens.next();

        cmd.map(|name| self.command_manager.launch_command(name, tokens.collect()))
            .flatten()
            .unwrap_or(Box::pin(CommandManager::fake_command()))
            .await
    }
}

impl Hinter for DefaultTerminal {
    type Hint = DefaultHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> std::option::Option<DefaultHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        let mut tokens: Vec<_> = line.split_whitespace().collect();

        let pos = if line.ends_with(' ') { pos - 1 } else { pos };

        if tokens.len() == 1 {
            self.command_manager.get_command_hint(tokens[0], pos)
        } else if tokens.len() == 0 {
            Option::None
        } else {
            let command = tokens.remove(0);
            self.command_manager
                .get_command_args_hint(command, &tokens, pos)
        }
    }
}

#[derive(Default)]
pub struct DefaultTerminalBuilder {
    command_manager: CommandManager,
}

impl DefaultTerminalBuilder {
    pub fn commands(mut self, commands: Vec<Box<dyn Command>>) -> DefaultTerminalBuilder {
        let cm_builder = CommandManagerBuilder::new();
        self.command_manager = cm_builder.commands(commands).build();
        self
    }

    pub fn build(self) -> DefaultTerminal {
        DefaultTerminal {
            command_manager: self.command_manager,
        }
    }
}
