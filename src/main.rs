use std::env;
use std::path::Path;
use std::fs::File;

fn main() {
	let args: Vec<String> = env::args().collect();
	if args.len() > 1 {
		let mut replaced:String = args[1]
			.clone().to_ascii_lowercase()
			.replace(".", "").replace(" ", "_");
		replaced.push_str(".rs");
		let mut path_string:String = "./leetcode/".to_owned();
		path_string.push_str(&replaced);
		println!("replaced {}", replaced);
		println!("replaced file name copy to clipboard");
		let path = Path::new(&path_string);
		if path.exists() {
			println!("file already existed");
		} 
		else {
			let display = path.display();
			match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(_file) => println!("create file success"),
    	};
		}
	}
	else {
		eprintln!("no file name args");
	}
}