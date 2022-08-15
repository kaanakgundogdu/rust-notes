struct Person{
    name: String
}

impl Person{
    // fn new(name: &str) -> Person{
    //     Person { name: name.to_string() }
    // }
    fn new<S: Into<String>>(name:S)->Person{
        Person { name: name.into() }
    }
}

fn main() {
    let kaan = Person::new("Kaan");
    let n_name = "Penguen".to_string();
    let peng = Person::new(n_name);/*.as_ref()*/
    println!("{}",kaan.name);
    println!("{}",peng.name);
}
