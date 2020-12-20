use std::{thread, time};
use notifica::{{notify}};

fn main(){
	println!("Starting reminder...");
	execute(1800000, false) // Every 30 minutes
}

fn execute(delay: u64, log: bool){
	let ten_millis = time::Duration::from_millis(delay);
	loop {
		thread::sleep(ten_millis);
		let result = notify("Reminder", "Look away from the screen 😵");
		if log == true{
			match result {
				Ok(_) => println!("Sent notification"),
				Err(_) => println!("Error")
			}
		}
	}
}

#[test]
fn test(){
	execute(2000, true) // Every 2 seconds
}