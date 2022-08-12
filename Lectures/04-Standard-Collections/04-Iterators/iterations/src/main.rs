use std::vec;

fn iterators(){
    let mut vctr= vec![1,2,3];
    
    //not mutable iter
    for x in vctr.iter() {
        println!("normal {}",*x);
    } 
    // mutable iter
    for x in vctr.iter_mut() {
        *x*=2;
    }
    println!("modified: {:?}",vctr);

    for x in vctr.iter().rev() {
        println!("reverse: {}",*x);
    } 
    let mut vctr2= vec![3,2,1];
    vctr2.extend(vctr);
    println!("new vector: {:?}",vctr2);
    //now you cant use vctr anymore
    // println!("new vector: {:?}",vctr); //this gives error
    
}

fn main() {
    iterators();

}
