//used to find the best fit model of an ellipsoid
use std::f32::consts::PI;
use rayon::prelude::*;

pub fn ellipse(a1: f32, a2: f32, a3: f32, mu: f32) -> (f32, f32, f32, f32, f32, f32, f32, f32) {
    let volume = (4.0 / 3.0) * PI * a1 * a2 * a3;

    let loop_number = 100000.0;

    let normal = volume.cbrt();

    let a1 = a1 / normal;
    let a2 = a2 / normal;
    let a3 = a3 / normal;


    let ds = 0.1;
    let mut n1: Vec<f32> = Vec::new();
    let mut n2: Vec<f32> = Vec::new();
    let mut n3: Vec<f32> = Vec::new();


    let pow_a1 = a1.powf(2.0);
    let pow_a2 = a2.powf(2.0);
    let pow_a3 = a3.powf(2.0);

    let xm = mu - 1.0;

    let dump = vec![0.0, loop_number];
    let index:Vec<f32> = dump.par_iter().map(|x| x + ds).collect();

    
    
    let denominator = index.par_iter().map(|d|
        ((d + pow_a1) * (d + pow_a2) * (d + pow_a3)).sqrt()).collect();
   

        //need to finish making all of these as Rust iterators so they're easy to parallize
    n1.push( ds / ((s + pow_a1) * denominator));
    n2.push( ds / (( s+ pow_a2) * denominator));
    n3.push( ds / (( +s pow_a3) * denominator));
   
    
    let n1: f32 = n1.par_iter().sum();
    let n2: f32 = n2.par_iter().sum();
    let n3: f32 = n3.par_iter().sum();

    let n1 = n1 * (a1 * a2 * a3 / 2.0);
    let n2 = n2 * (a1 * a2 * a3 / 2.0);
    let n3 = n3 * (a1 * a2 * a3 / 2.0);

    //println!("n1  to: {:.3?}", n1*10000000.00);
    //println!("n2  to: {:.3?}", n2);
    //println!("n3  to: {:.3?}", n3);

    // println!("Volume is: {:.3?}", volume);

    let alpha1: f32 = volume * xm / (1.0 + xm * n1);
    let alpha2: f32 = volume * xm / (1.0 + xm * n2);
    let alpha3: f32 = volume * xm / (1.0 + xm * n3);

    return (alpha1, alpha2, alpha3, volume, a1, a2, a3, mu);
}
