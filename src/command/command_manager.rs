use super::Command;
use crate::hint::default_hint::DefaultHint;
use rustyline::hint::Hinter;
use rustyline::Context;
use rustyline_derive::{Completer, Helper, Highlighter, Validator};

#[derive(Default, Completer, Helper, Highlighter, Validator)]
pub(crate) struct CommandManager {
    pub commands: Vec<Box<dyn Command>>,
}

impl CommandManager {
    pub fn builder() -> CommandManagerBuilder {
        CommandManagerBuilder::default()
    }

    pub async fn run_command(&self, data: String) -> Vec<u8> {
        let mut tokens = data
            .split_whitespace()
            .map(|s| <String as std::str::FromStr>::from_str(s).unwrap());

        let cmd: Option<String> = tokens.next();

        cmd.map(|name| {
            self.commands
                .iter()
                .find(|command| command.get_name() == name)
                .map(|command| command.run(tokens.collect()))
        })
        .flatten()
        .unwrap_or(Box::pin(CommandManager::fake_command()))
        .await
    }

    fn fake_command() -> impl std::future::Future<Output = Vec<u8>> + Send {
        async { Default::default() }
    }

    fn get_command_hint(&self, tokens: &str, pos: usize) -> Option<DefaultHint> {
        self.commands
            .iter()
            .filter(|command| command.get_name().starts_with(tokens))
            .next()
            .map(|hint| DefaultHint {
                hint: hint.get_name()[pos..].to_string(),
                complete_up_to: hint.get_name().len().saturating_sub(pos),
            })
    }

    fn get_command_args_hint(
        &self,
        command_name: &str,
        args: &Vec<&str>,
        pos: usize,
    ) -> Option<DefaultHint> {
        self.commands
            .iter()
            .filter(|command| command.get_name() == command_name)
            .next()
            .map(|command| {
                args.last()
                    .map(move |input_seq| {
                        command
                            .get_args()
                            .iter()
                            .filter(|arg_name| arg_name.starts_with(input_seq))
                            .next()
                            .map(|hint| DefaultHint {
                                hint: hint[pos..].to_string(),
                                complete_up_to: hint.len().saturating_sub(pos),
                            })
                    })
                    .flatten()
            })
            .flatten()
    }
}

impl Hinter for CommandManager {
    type Hint = DefaultHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> std::option::Option<DefaultHint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        let mut tokens: Vec<_> = line.split_whitespace().collect();

        let pos = if line.ends_with(' ') { pos - 1 } else { pos };

        if tokens.len() == 1 {
            self.get_command_hint(tokens[0], pos)
        } else if tokens.len() == 0 {
            Option::None
        } else {
            let command = tokens.remove(0);
            self.get_command_args_hint(command, &tokens, pos)
        }
    }
}

#[derive(Default)]
pub(crate) struct CommandManagerBuilder {
    commands: Vec<Box<dyn Command>>,
}

impl CommandManagerBuilder {
    pub(crate) fn commands(mut self, commands: Vec<Box<dyn Command>>) -> CommandManagerBuilder {
        self.commands = commands;
        self
    }

    pub(crate) fn build(self) -> CommandManager {
        CommandManager {
            commands: self.commands,
        }
    }
}
