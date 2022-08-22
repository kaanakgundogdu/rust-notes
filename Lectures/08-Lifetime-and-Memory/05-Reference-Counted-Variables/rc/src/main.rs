use std::rc::Rc;

struct Person{
    name:Rc<String>
}

impl Person {
    fn new(name: Rc<String>) -> Person{
        Person { name: name }
    }
    fn greetings(&self) {
        println!("Hi my name is {}.",self.name);
    }
}


fn rc_demo(){
    let name = Rc::new("John Cena".to_string());
    println!("Name is {} and name has {} strong pointers.",name,Rc::strong_count(&name));
    {
        let john_cena=Person::new(name.clone());
        println!("Name is {} and name has {} strong pointers.",name,Rc::strong_count(&name));
        john_cena.greetings();
    }

    // println!("Name is {}.",name); // if you dont use rc this gives erro
    println!("Name is {} and name has {} strong pointers.",name,Rc::strong_count(&name));

}

fn main() {
    rc_demo();
}
