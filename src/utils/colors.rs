use colored::Colorize;

pub struct ColoredOutput;

impl ColoredOutput {
    pub fn success(msg: &str) {
        println!("{}", msg.green().bold());
    }

    pub fn error(msg: &str) {
        eprintln!("{}", msg.red().bold());
    }

    pub fn warning(msg: &str) {
        println!("{}", msg.yellow().bold());
    }

    pub fn info(msg: &str) {
        println!("{}", msg.blue());
    }

    pub fn debug(msg: &str) {
        println!("{}", msg.dimmed());
    }

    pub fn header(msg: &str) {
        println!("\n{}\n", msg.cyan().bold());
    }

    pub fn highlight(msg: &str) {
        println!("{}", msg.magenta().bold());
    }
}
