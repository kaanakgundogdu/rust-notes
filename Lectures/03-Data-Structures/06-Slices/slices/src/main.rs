
fn use_slice(slice: &mut[i32]){
    println!("First elenent is {}, len is {}",slice[0],slice.len());
    slice[0]=666;
}


fn slices(){
    let mut data = [1,2,3,4,5,6];

    use_slice(&mut data[1..4]);

    println!("{:?}",data);
}


fn main() {
    slices();
}
