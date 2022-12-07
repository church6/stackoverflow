// @filename   : stackoverflow_0193_26033976.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/26033976
// @title      : How do I create a Vec from a range and shuffle it?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Rust edition 2018 no longer needs extern crate
        use rand::seq::SliceRandom;
        use rand::thread_rng; // Shuffle a mutable slice in place.

        fn example() {
            let mut vec: Vec<usize> = (0..10).collect();
            vec.shuffle(&mut thread_rng());
            println!("{:?}", vec);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        fn example() {
            let mut vec: Vec<usize> = (0..10).collect();
            //let slice: &mut [usize] = &mut vec;
            //thread_rng().shuffle(slice);//fix
            vec.shuffle(&mut thread_rng());
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        fn example() {
            let mut vec: Vec<usize> = (0..10).collect();
            //let slice = vec.as_mut_slice();
            // thread_rng().shuffle(slice);//fix
            vec.shuffle(&mut thread_rng());
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code4 {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        fn example() {
            let mut vec: Vec<usize> = (0..10).collect();
            //thread_rng().shuffle(&mut vec);//fix
            vec.shuffle(&mut thread_rng());
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
        code4::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        use rand::seq::SliceRandom;
        use rand::thread_rng;

        fn example() {
            let mut vec: Vec<usize> = (0..10).collect();
            println!("{:?}", vec);
            //thread_rng().shuffle(&mut vec);//fix
            vec.shuffle(&mut thread_rng());
            println!("{:?}", vec);
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
