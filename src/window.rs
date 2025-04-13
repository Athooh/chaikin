// Remove or comment out the unused import
// use macroquad::prelude::*;

pub struct Window {
    #[allow(dead_code)]
    title: String,
}

impl Window {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
        }
    }
}