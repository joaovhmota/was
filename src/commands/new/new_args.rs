use clap::Args;

#[derive(Args, Debug)]
pub struct NewArgs {
    #[arg(help = "The name of the migrations to be created")]
    pub migrations_name: String,
}
