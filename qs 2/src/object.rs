use crate::position::Position;

#[derive( Clone)]
pub struct Object{
    name: String,
    x: i32,
    y: i32,
    children: Vec<Object>,
    parent: Option<Box<Object>>,
}

impl Object{
    pub fn new_with_x_y(_name: &str, _x: i32, _y: i32) -> Object {
        Object {
            name: _name.to_string(),
            x: _x,
            y: _y,
            children: Vec::new(),
            parent: None,
        }
    }
    pub fn new(_name: &str) -> Object{
        Object{
            name: _name.to_string(),
            x: 0,
            y: 0,
            children: Vec::new(),
            parent: None,
        }
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
    pub fn rename_to(&mut self, _name: &str){
      while self.name.len() > 0{
          self.name.remove(0);
      }
      self.name.push_str(_name)
    }
    pub fn x(&self) -> i32 {
        let mut x = self.x;
        if let Some(parent) = &self.parent{
            x = x + parent.x();
        }
        x
    }
    pub fn y(&self) -> i32 {
        let mut y = self.y;
        if let Some(parent) = &self.parent{
            y = y + parent.y();
        }
        y
    }
    pub fn set_x(&mut self, _x: i32){
        self.x = _x
    }
    pub fn set_y(&mut self, _y: i32){
        self.y = _y
    }

    // add a child if child with same name doesnot exist, return true if added
    pub fn add_child(&mut self,_child: &mut Object) -> bool{
        let mut exists = false;
        for child in self.children.iter(){
            if child.name() == _child.name(){
                exists = true;
            }
        }
        if !exists{
            self.children.push(_child.clone());
            _child.add_parent(&self);
        }
        !exists
    }

    // remove a child with matching name if exist, return true if removed
    pub fn remove_child(&mut self, _child: &mut Object) -> bool{
        let mut removed = false;
        let len = self.children.len();
        for i in 0..len{
            if self.children[i].name() == _child.name(){
                if i < len-1{  // if child is not last in array move it to _child's position and pop
                    self.children[i] = self.children[len-1].clone();
                }
                self.children.pop();
                _child.remove_parent();
                removed = true;
                break;
            }
        }
        removed
    }

    pub fn print_children(&self){
        for child in self.children.iter(){
            println!("{}",child.name());
        }
    }

    fn add_parent(&mut self,_parent: &Object) {
        let name = &self.name();
        if let Some(parent) = &mut self.parent{
            let mut y = Object::new(name);
            parent.remove_child(&mut y);
        }
        self.parent = Some(Box::new(_parent.clone()));
    }

    fn remove_parent(&mut self) {
        self.parent = None;
    }

    pub fn print_parent(&self){
        if let Some(parent) = &self.parent{
            println!("{}",parent.name());
        }
    }
}