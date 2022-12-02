// @filename   : stackoverflow_0093_39785597.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/39785597
// @title      : How do I get a slice of a Vec<T> in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            // https://doc.rust-lang.org/std/vec/struct.Vec.html#slicing
            let a = vec![1, 2, 3, 4, 5];

            // With a start and an end
            println!("{:?}", &a[1..4]);

            // With a start and an end, inclusive
            println!("{:?}", &a[1..=3]);

            // With just a start
            println!("{:?}", &a[2..]);

            // With just an end
            println!("{:?}", &a[..3]);

            // With just an end, inclusive
            println!("{:?}", &a[..=2]);

            // All elements
            println!("{:?}", &a[..]);
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
