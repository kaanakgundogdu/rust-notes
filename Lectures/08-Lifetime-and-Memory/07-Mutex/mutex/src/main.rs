use std::thread;
use std::sync::{Mutex,Arc};

struct Person{
    name:Arc<String>,
    state:Arc<Mutex<String>>
}

impl Person {
    fn new(name: Arc<String>,state: Arc<Mutex<String>>) -> Person{
        Person { name: name,state:state}
    }
    fn greetings(&self) {
        let mut mutable_state= self.state.lock().unwrap();
        mutable_state.clear();
        mutable_state.push_str("sad");
        println!("Hi my name is {}. {}",self.name,mutable_state.as_str());
    }
}


fn mutex_demo(){
    let name = Arc::new("John Cena".to_string());
    let state = Arc::new(Mutex::new("Dead Inside".to_string()));
    let cena=Person::new(name.clone(),state.clone());
    let t =thread::spawn(move || {
        cena.greetings();
    });
    println!("And his name is {}. And feeling {}",name,state.lock().unwrap().as_str());
    t.join().unwrap();
}

fn main() {
    mutex_demo();
}
