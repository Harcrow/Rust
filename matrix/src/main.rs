/* ultimate goal:
x
Well here's something that would be cool
(but a not small project), digest some tables of retrieved objects
(with polarizability values) and do a best fit of an ellipsoid model
to it and spit out the estimated permeability and semi axes.

test

SVD(conj_transpose(alpha) x alpha) <-- apparently what we do?

1. Measure
2. retrieve full polarizability tensor
3. derive rotation matrix
4. run a regression model plus threshold -- this extracts trends from the LUT to provide a score to each object in a scan
*/

use std::env;

mod ellipsoid_polarizability;
mod tensor_parse;

use ellipsoid_polarizability as ell;

fn main() {
    let args: Vec<String> = env::args().collect();

    let epsilon = 0.001;

    let path = &args[1];

    let vec = tensor_parse::get_file(path.to_string());
    let mat = tensor_parse::complex_matrix(vec);

    let svd = mat.svd(true, true);

    let ellipse = ell::ellipse(1.0, 2.0, 3.0, 4.0, 1.1);

    
    
    println!("Right-Singular Vector {:.3?}", svd.v_t);
    println!("Left-Singular Vector {:.3?}", svd.u);
    println!("Singular Values {:.3?}", svd.singular_values);
    println!("{:?}", ellipse);
    
}
