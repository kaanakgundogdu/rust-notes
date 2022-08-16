#[allow(unused_variables)]

fn main() {
    let v = vec![1,2,3];
    let v2=v;
    // print&&ln!("{:?}",v); //doesnt compile
    println!("{:?}",v2); // this works

    let n1=9;
    let n2=n1;
    println!("n1 is {}",n1);//this works fine
    println!("n2 is {}",n2);//this works fine

    let n3= Box::new(5);
    let n4=n3;
    // println!("n3 is {}",n3);//gives error
    let print_vector= |vectr:Vec<i32>|->Vec<i32>{
        println!("{:?}",vectr);
        vectr
    };
    let v3=print_vector(v2);
    // println!("v2's first element {}",v2[0]); // doesn't compile
    println!("v3's first element {}",v3[0]);
}
