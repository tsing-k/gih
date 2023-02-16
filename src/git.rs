#![allow(dead_code)]
use std::{process::Command, path::PathBuf};
use anyhow::{Result, Ok};

pub struct Git {
    git_path: PathBuf,
}

impl Git {
    pub fn new(git_path: &str) -> Self {
        let mut pathbuf = PathBuf::from(git_path);
        if !pathbuf.as_path().is_file() {
            pathbuf.clear();
        }

        Git { git_path: pathbuf, }
    }

    pub fn default() -> Self {
        Git { git_path: PathBuf::from("git") }
    }

    fn git(&self, args: &[&str]) -> Result<String> {
        let output = Command::new(self.git_path.as_os_str())
            .args(args)
            .output()?;
    
        if output.status.success() {
            let output = String::from_utf8(output.stdout)?;
            Ok(output)
        } else {
            let err = String::from_utf8(output.stderr)?;
            Err(anyhow::anyhow!(err))
        }
    }

    pub fn branch(&self, exclude_self: bool) -> Result<Vec<String>> {
        let mut br_list = Vec::<String>::new();
        let output = self.git(&["branch"])?;

        for line in output.lines() {
            let line = line.trim();
            if exclude_self && line.starts_with('*') {
                continue;
            }
            br_list.push(String::from(line));
        }

        Ok(br_list)
    }

    pub fn branch_aa(&self, exclude_self: bool) -> Result<Vec<String>> {
        let mut br_list = Vec::<String>::new();
        let output = self.git(&["branch", "-aa"])?;

        for line in output.lines() {
            let line = line.trim();
            if exclude_self && line.starts_with('*') {
                continue;
            }
            br_list.push(String::from(line));
        }

        Ok(br_list)
    }

    pub fn delete_branch(&self, branch_list: &[&str]) -> Result<()> {
        let branches = branch_list.join(" ");
        self.git(&["branch", "-D", branches.as_str()])?;

        Ok(())
    }

    pub fn pull_branch(&self, branch: &str) -> Result<()> {
        self.git(&["checkout", branch])?;
        Ok(())
    }
    
    pub fn status(&self) -> Result<String> {
        self.git(&["status"])
    }
}