#![allow(dead_code)]
use std::{process::Command, path::PathBuf};
use anyhow::Result;

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

    pub fn branch(&self) -> Result<Vec<String>> {
        let mut br_list = Vec::<String>::new();
        let output = self.git(&["branch"])?;

        for line in output.lines() {
            let line = line.trim();
            if !line.starts_with('*') {
                br_list.push(String::from(line));
            }
        }

        Ok(br_list)
    }
    
    pub fn status(&self) -> Result<String> {
        self.git(&["status"])
    }
}