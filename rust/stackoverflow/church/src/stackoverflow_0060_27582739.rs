// @filename   : stackoverflow_0060_27582739.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27582739
// @title      : How do I create a HashMap literal?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Rust 1.56
        // Many collections now offer conversions from an array argument using From or Into:
        use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
        pub fn test() {
            // add your code here
            let s = Vec::from([1, 2, 3]);
            println!("{:?}", s);

            let s = BTreeSet::from([1, 2, 3]);
            println!("{:?}", s);

            let s = HashSet::from([1, 2, 3]);
            println!("{:?}", s);

            let s = BTreeMap::from([(1, 2), (3, 4)]);
            println!("{:?}", s);

            let s = HashMap::from([(1, 2), (3, 4)]);
            println!("{:?}", s);
        }
    }
    mod code2 {
        // This logic can be wrapped back into a macro for some syntax sugar:

        use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

        macro_rules! collection {
    // map-like
    ($($k:expr => $v:expr),* $(,)?) => {{
        core::convert::From::from([$(($k, $v),)*])
    }};
    // set-like
    ($($v:expr),* $(,)?) => {{
        core::convert::From::from([$($v,)*])
    }};
}
        pub fn test() {
            // add your code here
            let s: Vec<_> = collection![1, 2, 3];
            println!("{:?}", s);

            let s: BTreeSet<_> = collection! { 1, 2, 3 };
            println!("{:?}", s);

            let s: HashSet<_> = collection! { 1, 2, 3 };
            println!("{:?}", s);

            let s: BTreeMap<_, _> = collection! { 1 => 2, 3 => 4 };
            println!("{:?}", s);

            let s: HashMap<_, _> = collection! { 1 => 2, 3 => 4 };
            println!("{:?}", s);
        }
    }
    mod code3 {
        // Previous versions(<Rust 1.51)
        macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert($key, $value);
            )+
            m
        }
     };
);
        pub fn test() {
            // add your code here
            let names = map! { 1 => "one", 2 => "two" };
            println!("{} -> {:?}", 1, names.get(&1));
            println!("{} -> {:?}", 10, names.get(&10));
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
