
fn main() {
    let name ="Kaan";
    let greeting=format!("Hi, I'm {}, nice to meet you all!",name);
    println!("{}",greeting);

    let info = format!(
        "And his name is {last}. {first} {last}.",
        first="Jhon",
        last="Cena"
    );
    println!("{}",info);
}
