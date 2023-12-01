use std::{fs, path::Path};

pub trait Input {
    fn get_data(&self) -> String;
}

impl Input for String {
    fn get_data(&self) -> String {
        self.clone()
    }
}

impl Input for str {
    fn get_data(&self) -> String {
        String::from(self)
    }
}

pub struct Filename(pub String);

impl Input for Filename {
    fn get_data(&self) -> String {
        String::from(fs::read_to_string(Path::new(&self.0)).unwrap().get_data())
    }
}
