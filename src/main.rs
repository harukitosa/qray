mod cirq;
// use ndarray::*;
// use num_complex::{Complex, Complex64};
pub use crate::cirq::Circuit;

// fn x(input: Array2<Complex64>) -> Array2<Complex64> {
//     let x_gate = array![
//         [num_complex::Complex::<f64>::new(0., 0.), num_complex::Complex::<f64>::new(1., 0.)],
//         [num_complex::Complex::<f64>::new(1., 0.), num_complex::Complex::<f64>::new(0., 0.)],
//     ];
//     Array2::dot(&x_gate, &input)
// }

// fn y(input: Array2<Complex64>) -> Array2<Complex64> {
//     let y_gate = array![
//         [num_complex::Complex::<f64>::new(0., 0.), num_complex::Complex::<f64>::new(0., -1.)],
//         [num_complex::Complex::<f64>::new(0., 1.), num_complex::Complex::<f64>::new(0., 0.)],  
//     ];
//     Array2::dot(&y_gate, &input)
// }

fn main() {
    let c = Circuit::new();
    println!("{:?}", c);
    let h = c.set(vec![0, 0, 1, 1]);
    println!("{:?}", h);
}