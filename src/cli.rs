use clap::{Parser, Subcommand, ArgGroup};

#[derive(Debug, Parser)]
#[command(name = "gih")]
#[command(about = "git tool", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// git branch
    #[command(group(ArgGroup::new("branch").required(true).args(["a", "d"])))]
    Br{
        /// list all remote branch
        #[arg(short = 'a', action = clap::ArgAction::Count)]
        a: u8,

        /// delete branch
        #[arg(short = 'D')]
        d: Option<String>,
    },

    /// git checkout
    Co{
        /// branch checkout
        branch: Option<String>,
    }
}