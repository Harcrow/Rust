extern crate csv;
#[allow(unused)]
#[allow(unused_imports)]
extern crate nalgebra as na;

//use std::env;
use std::fs;
//use  std::process;

// The number of items divided by the number of rows equals the
// number of columns.
fn main() {
   // use na::Complex;
   // use na::Matrix3;

    //use na::linalg::SVD;

    //let mut m1 = Matrix3::repeat(0);

    //let mut x = 0;
    //let mut y = 0;
    //let mut com = Complex { re: 0, im: 0 };
    //let args: Vec<String> = env::args().collect();
    let path = "/home/tyler/Documents/Rust/matrix/src/matrix.csv";
    //let file_path = &args[1];
    let csv_read = fs::read_to_string(path);
    let _csv_iter = csv_read.iter();
    /*    let record = result.expect("a CSV record");
            let one = record[0].parse::<i64>().unwrap();
            let two = record[1].parse::<i64>().unwrap();

            com.re = one;
            com.im = two;

            m1[(x, y)] = com;
            x += 1;
            if x >= 3 {
                x = 0;
                y += 1;
            }
    */
    // Print a debug version of the record.
    println!("{:?}", csv_read);
}
/*
let xx = Complex {

re:698.680727128154,
im: -119.466079161084
};
let xy = Complex {
re:319.715801752267,
im:-96.7795868639316
};
let xz = Complex {
re:-148.105278843825,
    im:26.9792488607091
};
let yy = Complex {
re:631.430248485933,
im:-91.1754756430371
};
let yz = Complex {
re:-58.9247820986085,
im:27.8600860073641
};
let zz = Complex {
re:278.889024385914,
im: -21.3584451958786
};
*/
//   let m1 = Matrix3::new(xx, xy, xz,
//                       xy, yy, yz,
//                         xz, yz, zz );

// let svd1 = SVD::new(m1, true, true);
// let _svd2 = m1.svd(true, true);
// println!("{:.4?}" ,svd1);
