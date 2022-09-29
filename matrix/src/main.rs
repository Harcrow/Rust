#[allow(unused)]
#[allow(unused_imports)]
extern crate nalgebra as na;
extern crate csv;

//use std::env;
use std::io;
//use  std::fs;
//use  std::process;

fn main() {
    use na::Matrix3;
    use na::Complex;
//    use na::linalg::SVD;

//I think I need to add/create some values to initially fill this?s
    let mut m1 = Matrix3::zeros();
    let mut x = 0;
    let mut y = 0;

    //let args: Vec<String> = env::args().collect();

    //let file_path = &args[1];

    
    let mut rdr = csv::ReaderBuilder::new()
	.has_headers(false)
	.from_reader(io::stdin());
    // Loop over each record.
     for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
         // We will make this more friendly lat

	 let record = result.expect("a CSV record");
	 let mut com = Complex{re:record[0], im: record[1]};
	  m1[(x,y)] =com;
	 x += 1;
	 if x >= 3 {
	   x = 0;
	    y +=1;
	}
	
        // Print a debug version of the record.
       // println!("{:?}", m1);
	 }

	 println!("{:4?}", m1);

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
