

struct Person{
    name:String
}
impl Person {
    // fn get_ref_name(&self) ->&String {
    fn get_ref_name<'a>(&'a self) ->&'a String {
        &self.name
    }
}

struct Company<'z>{
    name:String,
    ceo:&'z Person
}

fn main() {
    
    let z: &String;
    {
        let boss=Person{name:String::from("Kaan Akg")};
        z=boss.get_ref_name();
        println!("{:?}",z);
    }

}
