// @filename   : stackoverflow_0119_22264502.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/22264502
// @title      : In Rust, what is the difference between clone() and to_owned()?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example() {
            test_clone();
            test_to_owned();
        }

        // compiles and runs fine
        #[allow(clippy::clone_double_ref)]
        fn test_clone() {
            let s1: &'static str = "I am static";
            let s2 = "I am boxed and owned".to_string();

            //let c1 = s1.clone();
            let c1 = s1.clone();
            let c2 = s2.clone();

            println!("{:?}", c1);
            println!("{:?}", c2);

            println!("{:?}", c1 == s1); // prints true
            println!("{:?}", c2 == s2); // prints true
        }

        fn test_to_owned() {
            let s1: &'static str = "I am static";
            let s2 = "I am boxed and owned".to_string();

            let c1 = s1.to_owned();
            let c2 = s2.to_owned();

            println!("{:?}", c1);
            println!("{:?}", c2);

            println!("{:?}", c1 == s1); // c1 is String whereas s1 is &str
            println!("{:?}", c2 == s2);
        }

        #[test]
        fn name() {
            let s1 = String::from("hello ");
            assert_eq!(s1, s1.clone());
            assert_eq!(s1, s1.to_owned());
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
        code1::test();
        //code2::test();
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
