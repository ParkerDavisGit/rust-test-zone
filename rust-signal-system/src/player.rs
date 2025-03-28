use std::fmt;

pub struct Player{
    x: f32,
    y: f32,
}


impl Player {
    pub fn new(new_x: f32, new_y: f32) -> Player {
        Player {
            x: new_x,
            y: new_y,
        }
    }

    pub fn move_to(&mut self, new_x: f32, new_y: f32) {
        self.x = new_x;
        self.y = new_y;
    }

    // fn test_func(&self) {
    //     self.move_to(23f32, 57f32);
    // }
}


impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}