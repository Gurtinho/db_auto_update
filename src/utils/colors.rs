use colored::Colorize;

pub enum Colors {
	Yellow,
	Blue,
	Red,
	Green,
	Purple
}

pub fn print_text_colored(text: &str, color: Colors) {
	match color {
		Colors::Yellow => println!("{}", text.yellow().bold().to_string()),
		Colors::Blue => println!("{}", text.blue().bold().to_string()),
		Colors::Red => println!("{}", text.red().bold().to_string()),
		Colors::Green => println!("{}", text.green().bold().to_string()),
		Colors::Purple => println!("{}", text.purple().bold().to_string()),
	}
}
