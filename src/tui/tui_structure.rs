pub struct App {
    pub input: String,
    pub result: Vec<String>,
    pub selected: usize,
}

impl App {
    pub fn new() -> Self {
        App { input: String::new(), result: Vec::new(), selected: 0 }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
