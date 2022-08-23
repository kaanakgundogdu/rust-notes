use std::thread;
use std::sync::Arc;

struct Person{
    name:Arc<String>
}

impl Person {
    fn new(name: Arc<String>) -> Person{
        Person { name: name }
    }
    fn greetings(&self) {
        println!("Hi my name is {}.",self.name);
    }
}


fn arc_demo(){
    let name = Arc::new("John Cena".to_string());
    let cena=Person::new(name.clone());
    let t =thread::spawn(move || {
        cena.greetings();
    });
    println!("And his name is {}",name);
    t.join().unwrap();
}

fn main() {
    arc_demo();
}
