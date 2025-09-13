use clap::Args;

#[derive(Args, Debug)]
pub struct InitArgs {
    #[arg(help = "Project's name")]
    pub name: String,

    #[arg(
        short = 'f',
        long = "force",
        help = "Forces the initialization of the project"
    )]
    pub force: bool,
}
