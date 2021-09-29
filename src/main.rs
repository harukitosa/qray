mod cirq;
pub use crate::cirq::Circuit;

fn main() {
    let c = Circuit::new(5);
    let c = c.set(vec![0, 1, 0, 1, 0]);
    println!("{}", c);
    c.measure();
}
