// @filename   : stackoverflow_0201_31216646.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/31216646
// @title      : Repeat string with integer multiplication

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Rust 1.16+
        // str::repeat is now available:

        fn example1() {
            let repeated = "Repeat".repeat(4);
            println!("{}", repeated);
        }

        // Rust 1.0+
        // You can use iter::repeat:

        use std::iter;

        fn example2() {
            #[allow(clippy::manual_str_repeat)]
            let repeated: String = iter::repeat("Repeat").take(4).collect();
            println!("{}", repeated);
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
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
        fn example() {
            println!("{:?}", (1..5).fold(String::new(), |b, _| b + "Repeat"));
        }

        pub fn test() {
            // add your code here
            example();
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
        code1::test();
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
