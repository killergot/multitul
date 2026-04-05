mod utils;

use crate::utils::git::GitProvider;

fn main(){
    println!("Hello, world!");

    GitProvider::new(".");
}