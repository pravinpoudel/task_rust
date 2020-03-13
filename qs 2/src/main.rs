mod position;
mod object;

fn main() {
    let mut parent = object::Object::new_with_x_y("parent", 10, 10);
    let mut child = object::Object::new_with_x_y("child", 5, 5);
    println!("{}::{}",child.x(),child.y());
    
    parent.add_child(&mut child);
    child.print_parent();
    println!("{}::{}",child.x(),child.y());

    let mut parent2 = object::Object::new_with_x_y("parent2", -10, -10);
    parent2.add_child(&mut child);
    child.print_parent();
    println!("{}::{}",child.x(),child.y());

    parent2.remove_child(&mut child);
    println!("{}::{}",child.x(),child.y());
}
