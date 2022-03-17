pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for com in self.components.iter() {
            com.draw();
        }
    }
}

pub struct Screens<T: Draw> {
    pub components: Vec<T>,
}
