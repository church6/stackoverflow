// @filename   : stackoverflow_0059_50251487.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/50251487
// @title      : What are non-lexical lifetimes?

#[allow(dead_code)]
mod answer1 {
    #[allow(unused_variables)]
    mod code1 {
        pub fn test() {
            // add your code here
            {
                let mut scores = vec![1, 2, 3];
                let score = &scores[0];
                scores.push(4);
                // error[E0502]: cannot borrow `scores` as mutable because it is also borrowed as immutable
                //println!("{}", score);
                println!("{:?}", scores);
            }
            {
                let mut scores = vec![1, 2, 3];
                let score = &scores[0];
                scores.push(4); // <-- score stops borrowing here
                                // error[E0502]: cannot borrow `scores` as mutable because it is also borrowed as immutable
                                //println!("{}", score);
                println!("{:?}", scores);
            }
        }
    }
    mod code2 {
        use std::collections::HashMap;
        fn example1(mut map: HashMap<i32, i32>, key: i32) {
            match map.get_mut(&key) {
                Some(value) => *value += 1,
                None => {
                    map.insert(key, 1);
                }
            }
            println!("{:?}", map);
        }
        fn example2(mut map: HashMap<i32, i32>, key: i32) {
            *map.entry(key).or_insert(0) += 1;
            println!("{:?}", map);
        }
        pub fn test() {
            // add your code here
            {
                let mut my_map = HashMap::new();
                my_map.insert(100i32, 1i32);
                my_map.insert(101i32, 3i32);
                println!("{:?}", my_map);
                //example1(my_map, 100i32);//value used here after move
            }

            {
                let mut my_map = HashMap::new();
                my_map.insert(100i32, 1i32);
                my_map.insert(101i32, 3i32);
                println!("{:?}", my_map);
                //example2(my_map, 101i32);//value used here after move
            }
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
