fn main(){
	println!("{} days" , 31);

	println!("{0}, this is {1}, {1}, this is {0}", "Alice", "Bob");
	
	println!("{subject} {verb} {object}",
	object = "ravana",
	verb = "killed",
	subject = "rama");

	println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

	println!("{number:>width$}", number = 1, width = 6);

	println!("{number:>0width$}", number = 1, width = 6);

	println!("My name is {0}, {1}, {0}", "Bond", "James");

	#[allow(dead_code)]
	struct Structure(i32);
	//println!(" This structure {} won't work", Structure(3));

	let pi  = 3.141592;
	println!("Pi = {:.*}", 3, pi);
}
	

