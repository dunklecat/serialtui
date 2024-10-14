pub(crate) mod command_manager;

use crate::hint::default_hint::DefaultHint;
use async_trait::async_trait;

#[async_trait]
pub trait Command {
    fn get_command(&self) -> &'static str;

    fn hint(&self, tokens: &Vec<&str>, pos: usize) -> Option<DefaultHint>;

    fn validate(&self, tokens: &Vec<&str>) -> bool {
        tokens.len() == 1 && tokens.last().unwrap() == &self.get_command()
    }

    async fn run(&self, args: Vec<String>) -> Vec<u8>;
}
