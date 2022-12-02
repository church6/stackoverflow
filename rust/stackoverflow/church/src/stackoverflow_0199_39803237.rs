// @filename   : stackoverflow_0199_39803237.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/39803237
// @title      : Build HashSet from a vector in Rust

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::collections::HashSet;
        use std::iter::FromIterator;

        fn hashset(data: &[u8]) -> HashSet<u8> {
            HashSet::from_iter(data.iter().cloned())
        }

        fn example() {
            let data: &[u8] = &[1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
            println!("{:?}", data);
            let hash_set: HashSet<u8> = hashset(data);
            println!("{:?}", hash_set);

            let mut sorted = hash_set.into_iter().collect::<Vec<_>>();
            sorted.sort();
            assert_eq!(sorted, vec![1, 2, 3, 4]);
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
        use std::collections::HashSet;
        use std::iter::FromIterator;

        fn vec_to_set(vec: Vec<u8>) -> HashSet<u8> {
            HashSet::from_iter(vec)
        }

        fn example() {
            let data: Vec<u8> = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
            println!("{:?}", data);
            let hash_set: HashSet<u8> = vec_to_set(data);
            println!("{:?}", hash_set);

            let mut sorted = hash_set.into_iter().collect::<Vec<_>>();
            sorted.sort();
            assert_eq!(sorted, vec![1, 2, 3, 4]);
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
        use std::collections::HashSet;
        // Moving data ownership
        fn example() {
            let vec: Vec<usize> = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
            println!("{:?}", vec);
            let hash_set: HashSet<usize> = vec.into_iter().collect();
            println!("{:?}", hash_set);

            let mut sorted = hash_set.into_iter().collect::<Vec<_>>();
            sorted.sort();
            assert_eq!(sorted, vec![1, 2, 3, 4]);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        use std::collections::HashSet;
        // Cloning data
        fn example() {
            let vec: Vec<usize> = vec![1, 2, 2, 3, 3, 3, 4, 4, 4, 4];
            println!("{:?}", vec);
            let hash_set: HashSet<usize> = vec.iter().cloned().collect();
            println!("{:?}", hash_set);

            let mut sorted = hash_set.into_iter().collect::<Vec<_>>();
            sorted.sort();
            assert_eq!(sorted, vec![1, 2, 3, 4]);
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
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    _leave!();
}