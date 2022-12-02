// @filename   : stackoverflow_0258_28678615.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28678615
// @title      : Efficiently insert or replace multiple elements in the middle or at the beginning of a Vec?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here

            let mut vec = vec![1, 5];
            let slice = &[2, 3, 4];

            vec.splice(1..1, slice.iter().cloned());

            println!("{:?}", vec); // [1, 2, 3, 4, 5]
        }
    }
    mod code2 {
        pub fn test() {
            // add your code here

            // Replacing a set of items

            let mut vec = vec![0, 1, 5];
            let slice = &[2, 3, 4];

            vec.splice(..2, slice.iter().cloned());

            println!("{:?}", vec); // [2, 3, 4, 5]
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here

            // Getting the previous values

            let mut vec = vec![0, 1, 2, 3, 4];
            let slice = &[9, 8, 7];

            let old: Vec<_> = vec.splice(3.., slice.iter().cloned()).collect();

            println!("{:?}", vec); // [0, 1, 2, 9, 8, 7]
            println!("{:?}", old); // [3, 4]
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
