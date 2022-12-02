// @filename   : stackoverflow_0117_34969902.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/34969902
// @title      : How to write a Rust function that takes an iterator?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::collections::HashMap;

        fn find_min<'a, I>(vals: I) -> Option<&'a u32>
        where
            I: Iterator<Item = &'a u32>,
        {
            vals.min()
        }

        fn example() {
            let mut map = HashMap::new();
            map.insert("zero", 0u32);
            map.insert("one", 1u32);
            println!("Min value {:?}", find_min(map.values()));
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        use std::collections::HashMap;

        fn find_min<'a, I>(vals: I) -> Option<&'a u32>
        where
            I: IntoIterator<Item = &'a u32>,
        {
            vals.into_iter().min()
        }

        fn example() {
            let mut map = HashMap::new();
            map.insert("zero", 0u32);
            map.insert("one", 1u32);
            println!("Min value {:?}", find_min(map.values()));
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
        use std::collections::HashMap;

        fn find_min<'a>(vals: impl Iterator<Item = &'a u32>) -> Option<&'a u32> {
            vals.min()
        }

        fn example() {
            let mut map = HashMap::new();
            map.insert("zero", 0u32);
            map.insert("one", 1u32);
            println!("Min value {:?}", find_min(map.values()));
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
