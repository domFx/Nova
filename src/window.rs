struct Window {
    width: u32,
    height: u32,
    is_running: bool, 
}

impl Window {
    pub fn on_resize(&self, width: i32, height: i32) {
        self.height = height;
        self.width = width;
    }

    pub fn run() { 
        
    }
}