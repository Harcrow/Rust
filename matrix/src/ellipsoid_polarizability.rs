//used to find the best fit model of an ellipsoid
use std::f32::consts::PI;

pub fn ellipse(a1: f32, a2: f32, a3: f32, mu: f32) -> (f32, f32, f32) {
    let volume = (4.0 / 3.0) * PI * a1 * a2 * a3;
    let normal = volume.cbrt();
    let a1 = a1 / normal;
    let a2 = a2 / normal;
    let a3 = a3 / normal;
    //let normalize = (volume.cbrt()).powf(2.0);

    //let semi_axis = vec![a1, a2, a3];
    // let mut normal_axis: Vec<f32> = Vec::new();

    // for i in &mut normal_axis {
    //    *i *= normalize;
    // }
    /*
    println!("a1  to: {:.3?}", a1);
    println!("a2  to: {:.3?}", a2);
    println!("a3  to: {:.3?}", a3);
    */

    let ds = 1.0;
    let mut n1 = 0.0;
    let mut n2 = 0.0;
    let mut n3 = 0.0;

    for x in 1..10 {
        let s = x as f32 * ds;

        n1 = n1
            + ds / (s + a1.exp2() * ((s + a1.exp2()) * (s + a2.exp2()) * (s + a3.exp2())).sqrt());
        n2 = n2
            + ds / (s + a2.exp2() * ((s + a1.exp2()) * (s + a2.exp2()) * (s + a3.exp2())).sqrt());
        n3 = n3
            + ds / (s + a3.exp2() * ((s + a1.exp2()) * (s + a2.exp2()) * (s + a3.exp2())).sqrt());
    }
    n1 += n1 * (a1 * a2 * a3 / 2.0);
    n2 += n2 * (a1 * a2 * a3 / 2.0);
    n3 += n3 * (a1 * a2 * a3 / 2.0);

    //println!("n1  to: {:.3?}", n1*10000000.00);
    //println!("n2  to: {:.3?}", n2);
    //println!("n3  to: {:.3?}", n3);

    // println!("Volume is: {:.3?}", volume);

    let alpha1: f32 = volume * (mu - 1.0) / (1.0 + (mu - 1.0) * n1);
    let alpha2: f32 = volume * (mu - 1.0) / (1.0 + (mu - 1.0) * n2);
    let alpha3: f32 = volume * (mu - 1.0) / (1.0 + (mu - 1.0) * n3);

    return (alpha1, alpha2, alpha3);
}
