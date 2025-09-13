use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg()]
    pub name: String,
}
