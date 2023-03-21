pub mod sample_trait {

    pub trait Shape {
        fn calc_area(&self) -> f64;
        fn calc_perimeter(&self) -> f64;
        fn do_something();
    }



    
    pub struct Rectangle {
        pub width: f64,
        pub height: f64,
    }

    impl Shape for Rectangle {
        fn calc_area(&self) -> f64{
            self.width * self.height
        }
        fn calc_perimeter(&self) -> f64{
            self.width * 2.0 + self.height *2.0
        }
        fn do_something() {
            println!("This is Rectangle function");
        }
    }

    pub struct Circle {
        pub radius: f64,
    }

    impl Shape for Circle {
        fn calc_area(&self) -> f64{
            self.radius * self.radius * std::f64::consts::PI
        }
        fn calc_perimeter(&self) -> f64{
            self.radius * 2.0 * std::f64::consts::PI
        }
        fn do_something() {
            println!("This is Circle function");
        }

    }


}