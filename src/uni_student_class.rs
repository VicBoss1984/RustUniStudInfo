pub struct UniStudent {
	name: String,
    	age: u32,
    	uni_name: String,
    	gpa: f32,
    	inter_stat: bool
}

impl UniStudent {
	pub fn new(name: String, age: u32, uni_name: String, gpa: f32, inter_stat: bool) -> Self {
        	UniStudent{name, age, uni_name, gpa, inter_stat}
    	}

    	pub fn print_info(&self) {
        	println!("Name: {}", self.name);
        	println!("Age: {}", self.age);
        	println!("University name: {}", self.uni_name);
        	println!("GPA: {}", self.gpa);
        	println!("International Student Status: {}", self.inter_stat);
    	}
}