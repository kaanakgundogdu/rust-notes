

fn vectors(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    a.push(4);
    a.push(5);
    let idx:usize=0;

    println!("a vector is: {:?}",a);
    println!("a[0] vector is: {:?}",a[idx]);

    //option get
    match a.get(6) {
        Some(x)=>println!("a[6] is {}",x),
        None=>println!("Error there is no element")
    }

    for elemnt in &a  {
        println!("{}",elemnt);
    }

    let last_elem = a.pop();//Some(5)
    println!("Last element is {:?}, a = {:?}",last_elem,a)


}

fn main() {
    vectors();
}
