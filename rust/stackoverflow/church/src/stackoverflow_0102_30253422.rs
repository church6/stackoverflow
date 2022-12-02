// @filename   : stackoverflow_0102_30253422.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30253422
// @title      : How to print structs and arrays?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[derive(Debug)]
        struct MyStruct {
            a: i32,
            b: i32,
        }
        pub fn test() {
            // add your code here
            let x = MyStruct { a: 10, b: 20 };
            println!("{:?}", x);
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
        code1::test();
        //code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        struct MyStruct {
            a: i32,
            b: i32,
        }

        impl std::fmt::Display for MyStruct {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "(value a: {}, value b: {})", self.a, self.b)
            }
        }
        pub fn test() {
            // add your code here
            let test = MyStruct { a: 0, b: 0 };
            println!("Used Display: {}", test);
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
        code1::test();
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
    answer2::test();
    //answer3::test();
    _leave!();
}
