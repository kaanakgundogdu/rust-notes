
fn if_statement(){
    let temp=32;
    if temp>30 {
        println!("Its hot outside {} degre!",temp);        
    }
    else if temp<10 {
        println!("Its really cold outside {} degre!",temp);
    }
    else {
        println!("Temperature is ok. {} degre!",temp);
    }
    
    let day= if temp>20 {"sunny"} else {"cloudy"};
    println!("Today is {}",day);

    println!("it is {} outside.",
    if temp>20 {"hot"} else if temp<10 {"colod"} else {"OK"});

    let new_temp=44;
    println!("it is {}", 
    if new_temp>30{
        if new_temp>40{"very hot"} else {"hot"}
    }
    else if new_temp<10 {"colod"} else {"OK"});
}

fn main() {
    if_statement();
}
