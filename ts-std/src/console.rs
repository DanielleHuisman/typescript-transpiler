pub struct Console {}

impl Console {
    pub fn log(&self, value: &str) {
        println!("{}", value);
    }
}

pub const console: Console = Console {};
