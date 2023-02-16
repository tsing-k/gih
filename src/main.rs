mod git;
use clap::{Parser, Subcommand, ArgGroup};
use git::Git;

#[derive(Debug, Parser)]
#[command(name = "gih")]
#[command(about = "git tool", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// git branch
    #[command(group(
        ArgGroup::new("branch")
            .required(true)
            .args(["a", "d"]),
    ))]
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
        branch: String,
    }
}

fn main() {
    // let git = Git::default();
    let args = Cli::parse();
    println!("{args:#?}");
    let git = Git::default();

    match args.command {
        Commands::Br{a, d} => {
            if a > 0 {
                match git.branch_aa(true) {
                    Ok(branch_list) => {
                        for branch in branch_list {
                            println!("{branch}");
                        }
                    },
                    Err(error) => println!("{error}"),
                }               
            } else if let Some(_branch) = d {
                
            }
        },
        Commands::Co { branch } => {
        },
    }
}
