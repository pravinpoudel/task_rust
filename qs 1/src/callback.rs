#[derive(Default)]
pub struct CallbackManager{
    callbacks: Vec<Box<dyn Fn()>>
}

impl CallbackManager{
    pub fn add(&mut self,closure:Box<dyn Fn()>){
        self.callbacks.push(closure);
    }
    pub fn run_all(&mut self){
        while let Some(cb) = self.callbacks.pop(){
            cb();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_and_run_all() {
        let mut cb_manager = CallbackManager::default();
        cb_manager.add(Box::new(||{println!("Callbackone");}));
        cb_manager.add(Box::new(||{println!("Callbacktwo");}));
        cb_manager.run_all();
    }
}