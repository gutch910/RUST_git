use trait_lesson::sample_trait::{Shape, Rectangle, Circle};

fn main() {
 
    let rect: Rectangle = Rectangle {
        width: 4.0,
        height: 5.0,
    };
    
    let circle: Circle = Circle{
        radius: 2.0,
    };

    println!("Rectangle area is: {}",rect.calc_area());
    println!("Rectangle perimeter is: {}",rect.calc_perimeter());
    Rectangle::do_something();


    println!("Circle area is: {}",circle.calc_area());
    println!("Circle perimeter is: {}",circle.calc_perimeter());
    Circle::do_something();
}
