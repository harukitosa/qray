use core::fmt;

use ndarray::*;
use num_complex;
use num_complex::{Complex, Complex64};
use rand::Rng;
#[derive(Debug)]
pub struct Circuit {
    name: String,
    size: usize,
    state_size: usize,
    qubits: ArrayBase<OwnedRepr<Complex64>, Dim<[usize; 2]>>,
}

impl fmt::Display for Circuit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut x = 0.0;
        for item in self.qubits.iter() {
		if item.re == 1.0 {
			break
		}
            x +=1.0;
        }
	let mut s = "".to_string();
        let str = &format!("{:b}", x as i32);
	s = s.clone() + str;
	for _i in 0..(self.size-str.len()) {
		let zero = "0".to_string();
		s = zero + &s;
	}
        write!(f, "|{}>", s)
    }
}

impl Circuit {
    pub fn new(n: usize) -> Self {
	    let s:usize = 2;
        let state: usize = s.pow(n as u32);
        let mut qubits = Array::<Complex64, _>::zeros((1, state).f());
        qubits[[0, 0]] = Complex::<f64>::new(1.0, 0.0);
        Self {
            name: "sample".to_string(),
            size: n,
            state_size: state,
            qubits,
        }
    }

    pub fn set(&self, v: Vec<u16>) -> Self {
        let mut qubits = Array::<Complex64, _>::zeros((1, self.state_size).f());
        let mut vec = v;
        vec.reverse();
        let mut idx = 0;
        let mut x = 1;
        for item in vec {
            idx += x * item;
            x *= 2;
        }
        qubits[[0, idx as usize]] = Complex::<f64>::new(1.0, 0.0);
        Self {
            name: "sample".to_string(),
            size: self.size,
            state_size: self.state_size,
            qubits,
        }
    }

    pub fn measure(&self) {
	    let i: f64 = rand::thread_rng().gen(); 
	    println!("rnd:{}", &i);   
	    let mut sum: f64 = 0.0;
	    let mut idx = 0;
	    for item in self.qubits.iter() {
		    sum+=item.norm();
		    if i <= sum {
			    break;
		    }
		    idx+=1;
	    }

	    let mut s = "".to_string();
	    let str = &format!("{:b}", idx as i32);
	    s = s.clone() + str;
	    for _i in 0..(self.size-str.len()) {
		    let zero = "0".to_string();
		    s = zero + &s;
	    }
	    println!("|{}>", s)
    }

    pub fn get_size(&self) -> usize {
	    return self.size
    }
}
