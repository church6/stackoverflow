// @filename   : stackoverflow_0090_25428920.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/25428920
// @title      : How to get a slice as an array in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Before Rust 2021, you need to import the trait:
        // use std::convert::TryInto;

        fn pop1(barry: &[u8]) -> [u8; 3] {
            barry.try_into().expect("slice with incorrect length")
        }

        fn pop2(barry: &[u8]) -> &[u8; 3] {
            barry.try_into().expect("slice with incorrect length")
        }

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
mod answer2 {
    mod code1 {
        use std::convert::AsMut;
        #[derive(Debug)]
        struct Example {
            a: [i32; 4],
            b: [i32; 6],
        }

        fn clone_into_array<A, T>(slice: &[T]) -> A
        where
            A: Default + AsMut<[T]>,
            T: Clone,
        {
            let mut a = A::default();
            <A as AsMut<[T]>>::as_mut(&mut a).clone_from_slice(slice);
            a
        }
        pub fn test() {
            // add your code here

            let original = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

            let e = Example {
                a: clone_into_array(&original[0..4]),
                b: clone_into_array(&original[4..10]),
            };

            println!("{:?}", e);
        }
    }
    mod code2 {
        use std::convert::AsMut;

        fn copy_into_array<A, T>(slice: &[T]) -> A
        where
            A: Default + AsMut<[T]>,
            T: Copy,
        {
            let mut a = A::default();
            <A as AsMut<[T]>>::as_mut(&mut a).copy_from_slice(slice);
            a
        }
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
    //answer1::test();
    answer2::test();
    //answer3::test();
    _leave!();
}
