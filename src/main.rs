mod git;
use git::Git;

fn main() {
    let git = Git::default();
    let out = git.status().unwrap();
    println!("status:\n{out}");

    let out = git.branch().unwrap();
    for branch in out {
        println!("{branch}");
    }
}
