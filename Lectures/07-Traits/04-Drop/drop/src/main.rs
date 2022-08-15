
struct Creature{
    name:String
}

impl Creature {
    fn new(name: &str) -> Creature{
        println!("{} enters the game.",name);
        Creature { name: name.into() }
    }
}

impl Drop for Creature {
    fn drop(&mut self) {
        println!("{} is dead.",self.name);
    }
}

fn main() {
    let monster = Creature::new("Johan");
    println!("Game proceeds");
    drop(monster);
    println!("End of scope")
}
