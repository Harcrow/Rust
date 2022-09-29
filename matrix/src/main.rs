extern crate nalgebra as na;
extern crate csv;

use std::env;
//use std::io;
//use  std::fs;
//use  std::process;

fn main() {
    //    use na::Matrix3;
    //   use na::Complex;
//    use na::linalg::SVD;



    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    let mut rdr = csv::Reader::from_file(file_path);

    for result in rdr.records(){
	let record = result.expect("a CSV record");
	println!("{:?}", record);
    }
//    println!("{:?}", rdr);
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
    
}
