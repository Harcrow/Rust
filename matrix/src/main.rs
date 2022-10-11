//use std::env;
use std::fs;
//use std::error::Error;
//use std::io::BufRead;
use nalgebra::Matrix3;
use nalgebra::Complex;
fn main() {
      // initialize the number of rows to zero; we'll increment this
  // every time we encounter a newline in the input
    let path = "/home/tyler/Documents/Rust/matrix/src/matrix.csv";
    let file = fs::read_to_string(path)
	.expect("Could not read file, ding dong");
   // println!("{}", file);
//    let mut split = "".to_owned().;
    let mut val: Vec<&str> = file
	.split(|c| c == ',' || c == '\n')
	.collect();
    val.truncate(18);
    let mut float_val: Vec<f64> = Vec::new();

    
    for i in &val {
	//println!("{:?}", i.trim());
	float_val.push(i.trim().parse::<f64>().unwrap());
    }

    let  mat = Matrix3::<Complex<f64>>::new(
	Complex::<f64>::new(float_val[0],float_val[1]),
	Complex::<f64>::new(float_val[2],float_val[3]),
	Complex::<f64>::new(float_val[4],float_val[5]),
		
	Complex::<f64>::new(float_val[6],float_val[7]),
	Complex::<f64>::new(float_val[8],float_val[9]),
	Complex::<f64>::new(float_val[10],float_val[11]),

	Complex::<f64>::new(float_val[12],float_val[13]),
	Complex::<f64>::new(float_val[14],float_val[15]),
	Complex::<f64>::new(float_val[16],float_val[17]));
        
   // println!("{:?}", mat);

    let svd = mat.svd(true, true);

    println!("{:?}", svd);
    
}
