struct Point<T, U> {
    x: T,
    y: U,
    
}

impl<T, U> Point<T, U> {
    //fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, U> {        
        Point {
            x: self.x,
            y: self.y,
        }
    }
}

fn main() {
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c'};

    //let p3 = p1.mixup(p2);
    let p3 = p2.mixup(p1);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}