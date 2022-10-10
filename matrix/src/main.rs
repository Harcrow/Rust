//use std::env;
use std::fs;
//use std::error::Error;
//use std::io::BufRead;
//use nalgebra::Matrix3;
fn main() {
      // initialize the number of rows to zero; we'll increment this
  // every time we encounter a newline in the input
    let path = "/home/tyler/Documents/Rust/matrix/src/matrix.csv";
    let file = fs::read_to_string(path)
	.expect("Could not read file, ding dong");
    println!("{}", file);
//    let mut split = "".to_owned().;
      let value = file.split(",");
    println!("{:?}", value);
    /*
    for s in split {
	println!("{}", s);
	data.push(from_str(s.trim()));
    }*/
 
   // let matrix = Matrix3::from_iterator(split);

   // println!("{:?}", matrix);
    
}
