//used to find the best fit model of an ellipsoid
use num::pow;
use std::f32::consts::PI;

pub fn ellipse(volume: f32, a1: f32, a2: f32, a3: f32, mu: f32) -> (f32, f32, f32) {
    if volume == 0.0 {
        let volume = (4.0 / 3.0) * PI * a1 * a2 * a3;
    }
    if a1 == 0.0 {
        let a1 = volume / (4.0 / 3.0 * PI) / a2 / a3;
    }
    if a2 == 0.0 {
        let a2 = volume / (4.0 / 3.0 * PI) / a1 / a3;
    }
    if a3 == 0.0 {
        let a3 = volume / (4.0 / 3.0 * PI) / a2 / a1;
    }

    let normalize = (volume.cbrt()).powf(2.0);

    let semi_axis = vec![a1, a2, a3];
    let mut normal_axis: Vec<f32> = Vec::new();

    for i in &mut normal_axis {
        *i *= normalize;
    }

    let ds = 0.5;
    let mut n1 = 0.0;
    let mut n2 = 0.0;
    let mut n3 = 0.0;

    for x in 1..20000000 {
        let s = x as f32 * ds;

        /*

        matlab example

        N(1,:) = aa1.*aa2.*aa3/2 .* sum(ds./((s+aa1.^2).*sqrt((s+aa1.^2).*(s+aa2.^2).*(s+aa3.^2))));
        N(2,:) = aa1.*aa2.*aa3/2 .* sum(ds./((s+aa2.^2).*sqrt((s+aa1.^2).*(s+aa2.^2).*(s+aa3.^2))));
        N(3,:) = aa1.*aa2.*aa3/2 .* sum(ds./((s+aa3.^2).*sqrt((s+aa1.^2).*(s+aa2.^2).*(s+aa3.^2))));

        */

        n1 += ds / (s + a1.exp2() * ((s + a1.exp2()) * (s + a2.exp2()) * (s + a3.exp2())).sqrt());
        n2 += ds / (s + a2.exp2() * ((s + a1.exp2()) * (s + a2.exp2()) * (s + a3.exp2())).sqrt());
        n3 += ds / (s + a3.exp2() * ((s + a1.exp2()) * (s + a2.exp2()) * (s + a3.exp2())).sqrt());
    }
    n1 += n1 * (a1 * a2 * a3 / 2.0);
    n2 += n2 * (a1 * a2 * a3 / 2.0);
    n3 += n3 * (a1 * a2 * a3 / 2.0);

    let alpha1: f32 = volume * ((mu - 1.0) / (1.0 + (mu - 1.0) * n1));
    let alpha2: f32 = volume * ((mu - 1.0) / (1.0 + (mu - 1.0) * n2));
    let alpha3: f32 = volume * ((mu - 1.0) / (1.0 + (mu - 1.0) * n3));

    return (alpha1, alpha2, alpha3);
}
