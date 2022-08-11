
struct Point <T,V>{
    x:T,
    y:V
}

struct SingleTypePoint <T>{
    x:T,
    y:T
}

struct Line <T>{
    start:SingleTypePoint<T>,
    end: SingleTypePoint<T>
}

fn generics(){
    let a = Point{x:0.3,y:4};

    let b=SingleTypePoint{x:1.2, y:3.2};
    let c=SingleTypePoint{x:29.2, y:15.2};

    let line= Line{start:b,end:c};

    println!("{} {}",a.x,a.y);
    println!("{} {} {} {}",line.start.x,line.start.y,line.end.x,line.end.y);
}

fn main() {
    generics();
}
