#[derive(Default)]
pub struct Circuit {
	name: String
}


impl Circuit {
	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn get_name_mut(&mut self) -> &mut String {
		&mut self.name
	}
}