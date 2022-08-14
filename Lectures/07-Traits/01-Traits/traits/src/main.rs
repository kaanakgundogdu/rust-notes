
trait Animal {
    fn create(name: &'static str) -> Self;
    fn name(&self) -> &'static str;
    fn talk(&self){
        println!("{} cannot talk",self.name());
    }
}

struct Dog{
    name:&'static str
}
struct Cat{
    name:&'static str
}
impl Animal for Dog {
    fn create(name: &'static str) -> Dog {
        Dog { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says hav hav",self.name())
    }
}

impl Animal for Cat {
    fn create(name: &'static str) -> Cat {
        Cat { name: name }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn talk(&self) {
        println!("{} says miyav",self.name())
    }
}

trait Summable <T> {
    fn sum(&self) -> T;
}

impl Summable<i32> for Vec<i32>{
    fn sum(&self) -> i32 {
        let mut res = 0;
        for x in self {
            res += *x;
        }
        return res;
    }
}

fn main() {
    // let d = Dog{name:"Kopek"};
    // let d = Dog::create("Kopek");
    let d:Dog = Animal::create("Kopek");
    let c = Cat{name:"Kedi"};
    d.talk();
    c.talk();
    println!("Dog's name is {}",d.name());
    println!("Cat's name is {}",c.name());

    let vctr = vec![1,2,3];
    println!("sum = {}",vctr.sum());
}
