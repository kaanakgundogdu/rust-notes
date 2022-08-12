use std::collections::HashMap;

fn hm(){
    let mut shapes= HashMap::new();
    shapes.insert(String::from("triangle"), 3);
    shapes.insert(String::from("square"), 4);

    println!("a square has {} sides",shapes["square".into()]);

    for (k,v) in &shapes{
        println!("{}:{}",k,v);
    }

    shapes.insert("square".into(), 5);
    println!("{:?}",shapes);

    shapes.entry("circle".into()).or_insert(1);
    
    {
        let actual=shapes.entry("circle".into()).or_insert(2);
        *actual=0;
    }
    println!("{:?}",shapes);

}

fn main() {
    hm();
}
