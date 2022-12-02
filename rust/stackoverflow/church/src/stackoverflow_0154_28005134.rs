// @filename   : stackoverflow_0154_28005134.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28005134
// @title      : How do I implement the Add trait for a reference to a struct?

#[allow(dead_code)]
mod answer1 {
    /*
    impl<'a, 'b> Add<&'b Vector> for &'a Vector {
        type Output = Vector;

        fn add(self, other: &'b Vector) -> Vector {
            Vector {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }
    */
    mod code1 {
        #[derive(Debug)]
        pub struct Vector {
            x: i32,
            y: i32,
        }

        pub trait Add {
            fn add(self, other: Vector) -> Vector;
        }

        impl Add for Vector {
            fn add(self, other: Vector) -> Vector {
                Vector {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        fn example() {
            let v1 = Vector { x: 1, y: 2 };
            println!("v={:?}", v1);
            let v2 = Vector { x: 3, y: 4 };
            println!("v={:?}", v2);
            let v3 = v1.add(v2);
            println!("v={:?}", v3);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        #[derive(Debug)]
        pub struct Vector {
            x: i32,
            y: i32,
        }

        pub trait Add {
            fn add(self, other: &Vector) -> Vector;
        }

        impl Add for &Vector {
            fn add(self, other: &Vector) -> Vector {
                Vector {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }

        fn example() {
            let v1 = &Vector { x: 1, y: 2 };
            let v2 = &Vector { x: 3, y: 4 };
            let v3 = v1.add(v2);
            println!("v={:?}", v1);
            println!("v={:?}", v2);
            println!("v={:?}", v3);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        //code3::test();
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
    answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
