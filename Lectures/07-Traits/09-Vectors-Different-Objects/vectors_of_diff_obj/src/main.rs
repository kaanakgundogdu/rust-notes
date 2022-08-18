trait Animal {
    fn create(name: &'static str) -> Self where Self:Sized;
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

enum Creature {
    Dog(Dog),
    Cat(Cat)
}

fn main() {
    //USING ENUM
    let mut creatures=Vec::new();
    creatures.push(Creature::Cat(Cat{name:"Kedi"}));
    creatures.push(Creature::Dog(Dog{name:"Kopek"}));

    for c in creatures {
        match c {
            Creature::Cat(c)=>c.talk(),
            Creature::Dog(d)=>d.talk()
        }
    }

    //trait 
    let mut animals:Vec<Box<dyn Animal>>=Vec::new();
    animals.push(Box::new(Cat{name:"Tonton"}));
    animals.push(Box::new(Dog{name:"Uyuz"}));

    for a in animals {
        a.talk();
    }

}
