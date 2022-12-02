// @filename   : stackoverflow_0294_34555837.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/34555837
// @title      : Sort HashMap data by value

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::collections::HashMap;
        fn example() {
            let text = "a b c a b c a b c c c c c c c c";
            // Count the frequency of each letter
            let mut count: HashMap<char, u32> = HashMap::new();
            for c in text.to_lowercase().chars() {
                //fixed by church
                if c.is_alphabetic() {
                    *count.entry(c).or_insert(0) += 1;
                }
            }

            // Get a sorted (by field 0 ("count") in reversed order) list of the
            // most frequently used characters:
            // let mut count_vec: Vec<(&char, &u32)> = count.iter().collect();
            let mut count_vec: Vec<_> = count.iter().collect();
            count_vec.sort_by(|a, b| b.1.cmp(a.1));
            println!("{:?}", count_vec);
            assert_eq!(&('c'), count_vec[0].0);
            println!("Most frequent character in text: {}", count_vec[0].0);
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
        use std::collections::HashMap;
        fn example() {
            let text = "a b c a b c a b c c c c c c c c";
            // Count the frequency of each letter
            let mut count: HashMap<char, u32> = HashMap::new();
            for c in text.to_lowercase().chars() {
                //fixed by church
                if c.is_alphabetic() {
                    *count.entry(c).or_insert(0) += 1;
                }
            }

            let top_char = count.iter().max_by(|a, b| a.1.cmp(b.1)).unwrap();
            println!("{:?}", top_char);
            assert_eq!(&('c'), top_char.0);
            println!("Most frequent character in text: {}", top_char.0);
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
        use std::collections::BTreeMap;
        use std::collections::HashMap;
        fn example() {
            let text = "a b c a b c a b c c c c c c c c";
            // Count the frequency of each letter
            let mut count: HashMap<char, u32> = HashMap::new();
            for c in text.to_lowercase().chars() {
                *count.entry(c).or_insert(0) += 1;
            }

            let count_b: BTreeMap<&u32, &char> = count.iter().map(|(k, v)| (v, k)).collect();
            /*
            println!(
                "Most frequent character in text: {}",
                count_b.last_key_value().unwrap().1
            );
            */
            let opt_pair = count_b
                .iter() // get an iterator over the tree
                .max_by_key(|p| p.1) // check the value of each pair for the max
                .unwrap(); // unwrap the result
            println!("key: {}, value: {}", opt_pair.0, opt_pair.1);
            assert_eq!(&&(10u32), opt_pair.0);
            assert_eq!(&&('c'), opt_pair.1);
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
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    _leave!();
}
