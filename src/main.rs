use std::io::Write;
use std::env;
use std::fs;

fn main() {
    let args = env::args();
	let mut i = 0;
	let mut file = String::from("");
	for arg in args {
		if i == 1 {
			file = arg;
		}
		i += 1;
	}
	if i == 0 {
		return;
	}

    let contents = fs::read_to_string(file)
        .expect("Something went wrong reading the file");

	let mut bf_out = match fs::File::create("bf_out.bf") {
		Err(error) => panic!("couldnt open file bf output file error: {}", error),
		Ok(bf_out) => bf_out,
	};

    for chr in contents.chars() {
		for _ in 0..(chr as u8) {
			bf_out.write_all(b"+").expect("error, couldnt write to bf output file"); // increment cell to current char value
		}
		// print out char 
		bf_out.write_all(b".").expect("error, couldnt write to bf output file");
		// set cell back to 0
		bf_out.write_all(b"[-]").expect("error, couldnt write to bf output file");
	}
}
