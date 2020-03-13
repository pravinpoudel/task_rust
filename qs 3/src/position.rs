use std::rc::Rc;
use core::cell::RefCell;
pub struct Shared<T> {
  data: Rc<RefCell<T>>,
}
//Define the IntoIterator for Shared<T> for any T which implements IntoIterator as well:
impl <T> Shared<T> {
  fn new(t: T)-> Shared<T> {
      Shared{data: Rc::new(RefCell::new(t))}
  }
}

impl<T> IntoIterator for Shared<T> where T:IntoIterator{
  type Item = T;
  type IntoIter= Self
  fn into_iter(self)->Self{
    self
  }
}