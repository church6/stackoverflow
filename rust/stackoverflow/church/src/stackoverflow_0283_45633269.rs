// @filename   : stackoverflow_0283_45633269.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/45633269
// @title      : How do you declare an interface in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        trait Shape {
            fn area(&self) -> f32;
        }

        struct Circle {
            radius: f32,
        }

        impl Shape for Circle {
            fn area(&self) -> f32 {
                self.radius.powi(2) * std::f32::consts::PI
            }
        }

        struct Square {
            side: f32,
        }

        impl Shape for Square {
            fn area(&self) -> f32 {
                self.side.powi(2)
            }
        }

        fn example() {
            display_area(&Circle { radius: 1. });
            display_area(&Square { side: 1. });
        }

        fn display_area(shape: &dyn Shape) {
            println!("area is {}", shape.area())
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        trait Shape {
            fn area(&self) -> f32;
        }

        struct Circle {
            radius: f32,
        }

        impl Shape for Circle {
            fn area(&self) -> f32 {
                self.radius.powi(2) * std::f32::consts::PI
            }
        }

        struct Square {
            side: f32,
        }

        impl Shape for Square {
            fn area(&self) -> f32 {
                self.side.powi(2)
            }
        }

        fn example() {
            display_area(&Circle { radius: 1. });
            display_area(&Square { side: 1. });
        }

        // Static dispatch
        //When a trait is statically dispatched, there is no overhead at runtime. This is an equivalent of C++ templates; but where C++ uses SFINAE, the Rust compiler checks the validity using the "hints" we give to him:

        fn display_area(shape: &impl Shape) {
            println!("area is {}", shape.area())
        }
        //With impl Shape, we say to the compiler that our function has a generic type parameter that implements Shape, therefore we can use the method Shape::area on our shape.

        //In this case, like in C++ templates, the compiler will generate a different function for each different type passed in.

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        trait Shape {
            fn area(&self) -> f32;
        }

        struct Circle {
            radius: f32,
        }

        impl Shape for Circle {
            fn area(&self) -> f32 {
                self.radius.powi(2) * std::f32::consts::PI
            }
        }

        struct Square {
            side: f32,
        }

        impl Shape for Square {
            fn area(&self) -> f32 {
                self.side.powi(2)
            }
        }

        fn example() {
            display_area(&Circle { radius: 1. });
            display_area(&Square { side: 1. });
        }

        // Dynamic dispatch
        fn display_area(shape: &dyn Shape) {
            println!("area is {}", shape.area())
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code4 {
        trait Hello {
            fn say_hello(&self);
        }

        impl Hello for &'static str {
            fn say_hello(&self) {
                println!("Hello, {}!", *self)
            }
        }

        fn example() {
            "world".say_hello();
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code5 {
        trait Hello {
            fn say_hello(&self) {
                println!("Hello there!")
            }
        }

        impl Hello for i32 {}

        fn example() {
            123.say_hello(); // call default implementation
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code6 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
        code4::test();
        code5::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer3 {
    mod code1 {
        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
pub fn test() {
    _enter!();
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
