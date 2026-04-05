mod utils;

use crate::utils::git::GitProvider;

fn main(){
    println!("Hello, world!");

    let test = GitProvider::new(".git");
    test.get_all_branches();
}