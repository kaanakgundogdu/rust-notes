use std::collections::HashSet;


fn hashset(){
    let mut mangas =  HashSet::new();
    mangas.insert("one piece");
    mangas.insert("monster");
    println!("{:?}",mangas);//order may change
    mangas.insert("monster");
    println!("{:?}",mangas); // nothing changes 

    if mangas.insert("bleach"){
        println!("Yaay you added bleach!")
    }else {
        println!("you didnt add.")
    }
    println!("{:?}",mangas); 

    if mangas.remove("monster") {
        println!("You removed monster")
    }
    println!("{:?}",mangas); 

    //subset
    let _1_10: HashSet<_>=(1..=10).collect();
    let _2_8: HashSet<_>=(2..=8).collect();
    let _15_20: HashSet<_>=(15..=20).collect();
    println!(
        "is {:?} a subset of {:?}? {}",
        _2_8,_1_10,
        _2_8.is_subset(&_1_10)
    );

    //disjoint = no common elements
    println!(
        "is {:?} a disjoint of {:?}? {}",
        _2_8,_15_20,
        _2_8.is_disjoint(&_15_20)
    );

    //union
    println!(
        "{:?} a union {:?} = {:?}",
        _2_8,_15_20,
        _2_8.union(&_15_20)
    );


}

fn main() {
    hashset();
}
