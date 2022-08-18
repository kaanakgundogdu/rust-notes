fn main() {

    let print_vector = |x:&Vec<i32>|{
        println!("x is {:?}",x);
        
    };

    let v=vec![1,2,3];
    print_vector(&v);
    println!("v is {:?}",v);

    let mut a=33;
    let b = &mut a;

    *b+=2;
    println!("a is {}",a);

    let mut v2=vec![2,4,6];

    for i in &v2 {
        println!("i is {}",i);
        // v2.push(8);//gives error
    }

}
