
struct Point{
    x:f64,
    y:f64
}

struct Line {
    start:Point,
    end:Point
}

fn structures(){
    let p = Point { x:4.2, y:1.2 };
    println!("point is ({} , {})!",p.x,p.y);
    let p2 = Point { x:1.9, y:7.5 };
    let new_line=Line {start:p,end:p2};
    println!("new_line is start({} , {}) end ({} , {})!",new_line.start.x,new_line.start.y,new_line.end.x,new_line.end.y);
}


fn main() {
    structures();
}
