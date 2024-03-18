pub struct Console {}

impl Console {
    pub fn log(value: &str) {
        println!("{}", value);
    }
}

pub const console: Console = Console {};
