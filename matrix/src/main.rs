/* ultimate goal:

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
//use std::fs::File;

use std::env;
//use std::io::Error;
//use rayon::prelude::*;


use ellipsoid_polarizability as ell;

mod ellipsoid_polarizability;
mod tensor_parse;
fn vec_builder(start: f32, stop: f32, resolution: f32, asp_rat: f32) -> Vec<f32>
{
    let ratio = asp_rat * resolution;
    let mut n = start + ratio;
    let last = stop + ratio;
    
    let mut new_vector = Vec::new();
    new_vector.push(start);
    while n <= last {
        new_vector.push(n);
        n = n + ratio;   
    }

    new_vector
}
fn main() { 
    env::set_var("RUST_BACKTRACE", "1");
    let ellipse_iter: i32 = 50;
    //let tolerance:f32 = 5000.0;
    /* 
    let ellipse_resolution = 0.1;
    */

    //let args = Cli::parse();
    //let args: Vec<String> = env::args().collect();
    //let path = &args[1];
    let path:&str = "/home/tyler/Documents/Rust/matrix/src/matrix.csv";

    let perm_vec = vec![50.0];

    let vec = tensor_parse::get_file(path.to_string());

    let mat = tensor_parse::complex_matrix(vec);

    let svd = mat.svd(true, true);

    let singular_values_1 = svd.singular_values[(0, 0)];
    let singular_values_2 = svd.singular_values[(1, 0)];
    let singular_values_3 = svd.singular_values[(2, 0)];
    
    //Determine the aspect ratio of the SVD to generate more tailored iteration vectors
    let aspect_ratio_1:f32 = singular_values_1/(singular_values_1 + singular_values_2 + singular_values_3);
    let aspect_ratio_2:f32 = singular_values_2/(singular_values_1 + singular_values_2 + singular_values_3);
    let aspect_ratio_3:f32 = singular_values_3/(singular_values_1 + singular_values_2 + singular_values_3);

    //println!("ell_vec {:?}", ell_vec);

    /*
    for i in &ell_vec {
        for j in &ell_vec {
            for k in &ell_vec {
                for l in &perm_vec {
                    let ellipse = ell::ellipse(*i, *j, *k, *l as f32);
                    tensor_a1.push(ellipse.0);
                    tensor_a2.push(ellipse.1);
                    tensor_a3.push(ellipse.2);
                    volume.push(ellipse.3);
                    i_index.push(i);
                    j_index.push(j);
                    k_index.push(k);
                    l_index.push(l);
                    
                }
            }
        }
    }
    */

    let mut x_index:Vec<f32> = Vec::new();

    let index= 1..ellipse_iter;

    //vectors dependant on aspect ratio of the SVD
    let ell_vec_1:Vec<f32> = index.clone().map(|x:i32| x as f32 * aspect_ratio_1).collect();
    let ell_vec_2:Vec<f32> = index.clone().map(|x:i32| x as f32 * aspect_ratio_2).collect();
    let ell_vec_3:Vec<f32> = index.clone().map(|x:i32| x as f32 * aspect_ratio_3).collect();

    //loops through until it finds an ellipsoid whose value is larger than the SVD.  This should provide two values
    //that we can set as the upper and lower bound of the next decade of resolution
    for x in 0..ellipse_iter-1{
    /*
    return (alpha1, alpha2, alpha3, volume, a1, a2, a3, mu)
    */    
        let ellipse = ell::ellipse(ell_vec_1[x as usize], ell_vec_2[x as usize], ell_vec_3[x as usize], perm_vec[0]);
        
        x_index.push((x as i32) as f32);

        if ellipse.0 >= singular_values_1 && ellipse.1 >= singular_values_2 && ellipse.2 >= singular_values_3{
            println!("ellipse 0 {:.2?}", ellipse.0);
            println!("x_index[x] {:?}", x_index[x as usize]);
            break;
        }

    }
    println!("length of x_index {:?}", x_index.len());
    let upper_bound = ell_vec_1[(x_index.len()-0) as usize];
    let lower_bound = ell_vec_1[(x_index.len()-1) as usize];
    let next_vec_1:Vec<f32> = vec_builder(lower_bound as f32, upper_bound as f32, 0.1, aspect_ratio_1);


    println!("upper_bound {:?}", upper_bound);
    println!("lower_bound {:?}", lower_bound);

    let upper_bound = ell_vec_2[(x_index.len()-2) as usize];
    let lower_bound = ell_vec_2[(x_index.len()-3) as usize];
    let next_vec_2:Vec<f32> = vec_builder(lower_bound as f32, upper_bound as f32, 0.1, aspect_ratio_2);  
   
    let upper_bound = ell_vec_3[(x_index.len()-2) as usize];
    let lower_bound = ell_vec_3[(x_index.len()-3) as usize];
    let next_vec_3:Vec<f32> = vec_builder(lower_bound as f32, upper_bound as f32, 0.1, aspect_ratio_3);


   println!("next vec 1 {:.2?}", next_vec_1);
   println!("next vec 2 {:.2?}", next_vec_2);
   println!("next vec 3 {:.2?}", next_vec_3);
   x_index = Vec::new();



   let mut tensor_a1 = Vec::new();
   let mut tensor_a2 = Vec::new();
   let mut tensor_a3 = Vec::new();
   
   for x in 1..next_vec_1.len()-1{
    /*
    return (alpha1, alpha2, alpha3, volume, a1, a2, a3, mu)
    */    
        let ellipse = ell::ellipse(next_vec_1[x as usize], next_vec_2[x as usize], next_vec_3[x as usize],perm_vec[0]);
        
        x_index.push((x as i32) as f32);
        
        // tensor_a1.push(ellipse.0);
        // tensor_a2.push(ellipse.1);
        // tensor_a3.push(ellipse.2);

        if ellipse.0 >= singular_values_1 && ellipse.1 >= singular_values_2 && ellipse.2 >= singular_values_3{
            break;
        }

    }

    let upper_bound = next_vec_1[(x_index.len()-0) as usize];
    let lower_bound = next_vec_1[(x_index.len()-1) as usize];
    let next_vec_1:Vec<f32> = vec_builder(lower_bound as f32, upper_bound as f32, 0.01, aspect_ratio_1);


    println!("upper_bound {:?}", upper_bound);
    println!("lower_bound {:?}", lower_bound);

    let upper_bound = next_vec_2[(x_index.len()-2) as usize];
    let lower_bound = next_vec_2[(x_index.len()-3) as usize];
    let next_vec_2:Vec<f32> = vec_builder(lower_bound as f32, upper_bound as f32, 0.01, aspect_ratio_2);  
   
    let upper_bound = next_vec_3[(x_index.len()-2) as usize];
    let lower_bound = next_vec_3[(x_index.len()-3) as usize];
    let next_vec_3:Vec<f32> = vec_builder(lower_bound as f32, upper_bound as f32, 0.01, aspect_ratio_3);

    for x in 1..next_vec_1.len()-1{
        /*
        return (alpha1, alpha2, alpha3, volume, a1, a2, a3, mu)
        */    
            let ellipse = ell::ellipse(next_vec_1[x as usize], next_vec_2[x as usize], next_vec_3[x as usize],perm_vec[0]);
            
            x_index.push((x as i32) as f32);
            
            tensor_a1.push(ellipse.0);
            tensor_a2.push(ellipse.1);
            tensor_a3.push(ellipse.2);
    
            if ellipse.0 >= singular_values_1 && ellipse.1 >= singular_values_2 && ellipse.2 >= singular_values_3{
                break;
            }
    
        }

   println!("next vec 1 {:.2?}", next_vec_1);
   println!("next vec 2 {:.2?}", next_vec_2);
   println!("next vec 3 {:.2?}", next_vec_3);
   x_index = Vec::new();



    println!("tensor 1   {:.2?}", tensor_a1);
    println!("tensor 2   {:.2?}", tensor_a2);
    println!("tensor 3   {:.2?}", tensor_a3);

    println!("Singular Values {:.2?}", svd.singular_values);
    
    //let mut tensor_index_a1: Vec<usize> = Vec::new();
    /*
    let mut tensor_index_a2: Vec<usize> = Vec::new();
    let mut tensor_index_a3: Vec<usize> = Vec::new();
    */

    /*
    for (count, v) in tensor_a1.iter().enumerate() {
        if v <= &(singular_values_1 + tolerance) && v >= &(singular_values_1 - tolerance) {
            tensor_index_a1.push(count);
           // println!("COUNT: {:?}, V: {:?}", count, v);
        }
    }
    
    for (_count, v) in tensor_index_a1.iter().enumerate()
    {
    println!("========================================");
	println!("Index is --------- {:?}", v);
	
	println!("alpha1 ----------- {:?}", tensor_a1[*v]);
	println!("alpha2 ----------- {:?}", tensor_a2[*v]);
	println!("alpha3 ----------- {:?}", tensor_a3[*v]);

	println!("Volume ----------- {:?}", volume[*v]);
	println!("semi-axis a1  ---- {:?}", ell_vec_1[*v]);
	println!("semi-axis a2  ---- {:?}", ell_vec_2[*v]);
	println!("semi-axis a3  ---- {:?}", ell_vec_3[*v]);
	println!("permiability  ---- {:?}", perm_vec[0]);
	
	
	println!("Singular Values {:.3?}", svd.singular_values);
	println!("========================================");

    } */

    /*
    println!("Found {:?} matches for a1", tensor_index_a1.len());
    //println!("a1 {:?}, a2 {:?}, a3, {:?} ", tensor_a1[tensor_index_a1]
    for (_count, v) in tensor_index_a1.iter().enumerate(){
	if (tensor_a2[*v] <= singular_values_2 + tolerance) && tensor_a2[*v] >= (singular_values_2 - tolerance){
	    tensor_index_a2.push(*v);
	   // println!("found a2: {:?}", v);
	}
    }
     println!("Found {:?} matches for a2", tensor_index_a2.len());

    for (_count, v) in tensor_index_a2.iter().enumerate(){
	if (tensor_a3[*v] <= singular_values_3 + tolerance) && tensor_a3[*v] >= (singular_values_3 - tolerance){
	    tensor_index_a3.push(*v);
	    //println!("found a3: {:?}", v);
	}
     
    }

      println!("Found {:?} matches for a3", tensor_index_a3.len());
    */
  

/* for (_count, v) in tensor_index_a1.iter().enumerate(){
	println!("========================================");
	println!("Index is --------- {:?}", v);
	
	println!("alpha1 ----------- {:?}", tensor_a1[*v]);
	println!("alpha2 ----------- {:?}", tensor_a2[*v]);
	println!("alpha3 ----------- {:?}", tensor_a3[*v]);

	println!("Volume ----------- {:?}", volume[*v]);
	println!("semi-axis a1  ---- {:?}", i_index[*v]);
	println!("semi-axis a2  ---- {:?}", j_index[*v]);
	println!("semi-axis a3  ---- {:?}", k_index[*v]);
	println!("permiability  ---- {:?}", l_index[*v]);
	
	
	println!("Singular Values {:.3?}", svd.singular_values);
	println!("========================================");
    
     */


}