// @filename   : stackoverflow_0043_28512394.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28512394
// @title      : How to lookup from and insert into a HashMap efficiently?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // https://doc.rust-lang.org/stable/std/collections/hash_map/enum.Entry.html
        use std::collections::hash_map::Entry;
        use std::collections::HashMap;
        pub fn test() {
            // add your code here
            let mut map = HashMap::new();
            let key = "key";
            let value = "default";

            let values = match map.entry(key) {
                Entry::Occupied(o) => o.into_mut(),
                Entry::Vacant(v) => v.insert(value),
            };
            println!("{:?}", values);

            let values = map.entry(key).or_insert_with(|| value);
            println!("{:?}", values);

            let values = map.entry(key).or_insert(value);
            println!("{:?}", values);

            let values = map.entry(key).or_default();
            println!("{:?}", values);
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
