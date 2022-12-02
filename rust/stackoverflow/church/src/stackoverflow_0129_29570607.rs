// @filename   : stackoverflow_0129_29570607.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/29570607
// @title      : Is there a good way to convert a Vec<T> to an array?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // As of Rust 1.51 you can parameterize over an array's length.
        use std::convert::TryInto;

        fn demo<T, const N: usize>(v: Vec<T>) -> [T; N] {
            v.try_into().unwrap_or_else(|v: Vec<T>| {
                panic!("Expected a Vec of length {} but it was {}", N, v.len())
            })
        }
        fn example() {
            let v: Vec<i32> = vec![1, 2, 3, 4];
            println!("{:?}", v);
            let a = demo(v);
            assert!(a == [1, 2, 3, 4]);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        // As of Rust 1.48, each size needs to be a specialized implementation:
        use std::convert::TryInto;

        fn demo<T>(v: Vec<T>) -> [T; 4] {
            v.try_into().unwrap_or_else(|v: Vec<T>| {
                panic!("Expected a Vec of length {} but it was {}", 4, v.len())
            })
        }
        fn example() {
            let v: Vec<i32> = vec![1, 2, 3, 4];
            println!("{:?}", v);
            let a = demo(v);
            assert!(a == [1, 2, 3, 4]);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        // As of Rust 1.43:
        use std::convert::TryInto;

        fn demo<T>(v: Vec<T>) -> [T; 4] {
            let boxed_slice = v.into_boxed_slice();
            let boxed_array: Box<[T; 4]> = match boxed_slice.try_into() {
                Ok(ba) => ba,
                Err(o) => panic!("Expected a Vec of length {} but it was {}", 4, o.len()),
            };
            *boxed_array
        }
        fn example() {
            let v: Vec<i32> = vec![1, 2, 3, 4];
            println!("{:?}", v);
            let a = demo(v);
            assert!(a == [1, 2, 3, 4]);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
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
