mod git;
mod cli;

use clap::Parser;
use dialoguer::{FuzzySelect, MultiSelect};
use git::Git;
use cli::{Cli, Commands};

fn main() {
    let args = Cli::parse();
    let git = Git::default();

    match args.command {
        Commands::Br{a, d} => {
            if a > 0 {
                match git.branch_a(true) {
                    Ok(branch_list) => {
                        for branch in branch_list {
                            println!("{branch}");
                        }
                    },
                    Err(error) => println!("{error}"),
                }
            }

            if let Some(branch) = d {
                let branches: Vec<&str> = branch.split(char::is_whitespace).collect();
                match git.delete_branch(branches.as_slice()) {
                    Ok(()) => println!("delete {branch} success"),
                    Err(error) => println!("{error}"),
                }
            } else {
                match git.branch_a(true) {
                    Ok(branch_list) => {
                        let multi_select = MultiSelect::new()
                            .items(branch_list.as_slice())
                            .interact_opt();
                        match multi_select {
                            Ok(index_list) => {
                                if let Some(index_list) = index_list {
                                    let mut branches: Vec<&str> = Vec::new();
                                    for index in index_list {
                                        branches.push(branch_list[index].as_str());
                                    }

                                    match git.delete_branch(branches.as_slice()) {
                                        Ok(()) => println!("delete {branches:?} success"),
                                        Err(error) => println!("{error}"),
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
        Commands::Co { branch } => {
            if let Some(branch) = branch {
                if let Err(error) = git.checkout(branch.as_str()) {
                    println!("{error}");
                } else {
                    println!("checkout {branch} success");
                }
            } else {
                match git.branch_a(true) {
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
                                    if let Err(error) = git.checkout(branch) {
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
