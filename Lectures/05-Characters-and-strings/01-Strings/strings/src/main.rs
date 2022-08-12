fn strings(){
    let s:&'static str= "Kaan";

    for c in s.chars() {
        println!("{}",c);
    }
    if let Some(f_char) = s.chars().nth(0){
        println!("First char is: {}",f_char);
    }

    let mut letters=String::new();
    let mut a= 'a' as u8;
    while a <= ('z') as u8 {
        letters.push(a as char);
        letters.push_str(",");
        a+=1;
    }
    println!("Letters are: {}",letters);

    let mut selam = "hello world".to_string();
    selam.remove(0);
    selam.push_str("!!!");
    println!("{}",selam.replace("ello", "goodbye"));

}

fn main() {
    strings();
}
