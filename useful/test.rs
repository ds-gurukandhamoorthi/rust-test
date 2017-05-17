use std::process::Command;

fn main(){
	Command::new("sh")
		.arg("-c")
		.arg("echo hello")
		.spawn()
		.expect("failed to execute process");
}



