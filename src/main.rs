mod git;
mod cli;

use clap::Parser;
use dialoguer::FuzzySelect;
use git::Git;
use cli::{Cli, Commands};

fn main() {
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
            if let Some(branch) = branch {
                println!("checkout {branch}");
            } else {
                match git.branch_aa(true) {
                    Ok(branch_list) => {
                        let fuzzy_select = FuzzySelect::new()
                            .items(branch_list.as_slice())
                            .default(0)
                            .interact_opt();
                        match fuzzy_select {
                            Ok(index) => {
                                if let Some(index) = index {
                                    let prefix = "remotes/origin/";
                                    let mut start_index = 0;
                                    if branch_list[index].starts_with(prefix) {
                                        start_index = prefix.len();
                                    }
                                    let branch = &branch_list[index][start_index..];
                                    if let Err(error) = git.pull_branch(branch) {
                                        println!("{error}");
                                    } else {
                                        println!("checkout {branch} success");
                                    }
                                }
                             },
                            Err(error) => println!("{error}"),
                        }
                    },
                    Err(error) => println!("{error}"),
                }
            }
        },
    }
}
