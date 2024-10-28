pub(crate) mod command_manager;

use async_trait::async_trait;

#[async_trait]
pub trait Command {
    fn get_name(&self) -> &'static str;
    fn get_args(&self) -> Vec<&'static str>;

    fn validate(&self, tokens: &Vec<&str>) -> bool {
        tokens.len() == 1 && tokens.last().unwrap() == &self.get_name()
    }

    async fn run(&self, args: Vec<String>) -> Vec<u8>;
}
