#[allow(dead_code)]
#[allow(unused_variables)]

//you have to specify lifetime for struct and implementations 
struct Person<'a>{
    name: &'a str
}

impl<'b> Person<'b> {
    fn talk(&self) {
        println!("Hi, my name is {}.",self.name);
    }
}

fn main() {
    let kaan = Person{name:"Kaan"};
    kaan.talk();
}
