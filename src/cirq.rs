use ndarray::*;

#[derive(Debug)]
pub struct Circuit {
	name: String,
	n: usize,
	qubits: ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
}

impl Circuit {
	pub fn new() -> Self {
		let number:i32 = 2;
		let n:usize = number.pow(2) as usize;
		let mut qubits = Array::<f64, _>::zeros((1, n).f());
		qubits[[0, 0]] = 1.0;
		Self {name: "sample".to_string(), n, qubits}
	}

	pub fn set(&self, v: Vec<u16>) -> Self {
		let mut qubits = Array::<f64, _>::zeros((1, self.n).f());
		let mut vec = v;
		vec.reverse();
		let mut idx = 0;
		let mut x = 1;
		for item in vec {
			println!("{}", item);
			idx += x * item;
			x *= 2;
		}
		qubits[[0, idx as usize]] = 1.0;
		Self {name: "sample".to_string(), n: self.n, qubits}
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn get_name_mut(&mut self) -> &mut String {
		&mut self.name
	}
}