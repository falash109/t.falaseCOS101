use std::fs;

fn main(){
   fs::remove_file("data.txt").expect("could not remoive file");
   println!("file is removed");
}