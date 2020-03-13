#[derive(Clone)]
pub struct Position{
    x: i32,
    y: i32,
}

impl Position{
    pub fn new() -> Position{
        Position{
            x:0,
            y:0,
        }
    }
    pub fn new_with_x_y(_x: i32, _y: i32) -> Position{
        Position{
            x: _x,
            y: _y,
        }   
    }
    pub fn x(&self) -> i32{
        self.x
    }
    pub fn y(&self) -> i32{
        self.y
    }
    pub fn set_x(&mut self, _x: i32){
        self.x = _x
    }
    pub fn set_y(&mut self, _y: i32){
        self.y = _y
    }
}