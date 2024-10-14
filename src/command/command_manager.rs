use std::{future::Future, pin::Pin};

use crate::hint::default_hint::DefaultHint;

use super::Command;

#[derive(Default)]
pub(crate) struct CommandManager {
    pub commands: Vec<Box<dyn Command>>,
}

impl CommandManager {
    pub(crate) fn launch_command(
        &self,
        name: String,
        args: Vec<String>,
    ) -> Option<Pin<Box<dyn Future<Output = Vec<u8>> + Send + '_>>> {
        self.commands
            .iter()
            .find(|command| command.get_command() == name)
            .map(|command| command.run(args))
    }

    pub(crate) fn fake_command() -> impl std::future::Future<Output = Vec<u8>> + Send {
        async { Default::default() }
    }

    pub(crate) fn get_command_hint(&self, tokens: &str, pos: usize) -> Option<DefaultHint> {
        self.commands
            .iter()
            .filter(|command| command.get_command().starts_with(tokens))
            .next()
            .map(|hint| DefaultHint {
                hint: hint.get_command()[pos..].to_string(),
                complete_up_to: hint.get_command().len().saturating_sub(pos),
            })
    }

    pub(crate) fn get_command_args_hint(
        &self,
        command: &str,
        args: &Vec<&str>,
        pos: usize,
    ) -> Option<DefaultHint> {
        self.commands
            .iter()
            .filter(|cmd| cmd.get_command() == command)
            .next()
            .map(|cmd| cmd.hint(args, pos))
            .flatten()
    }
}

#[derive(Default)]
pub(crate) struct CommandManagerBuilder {
    commands: Vec<Box<dyn Command>>,
}

impl CommandManagerBuilder {
    pub(crate) fn new() -> CommandManagerBuilder {
        CommandManagerBuilder {
            commands: Vec::new(),
        }
    }

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
