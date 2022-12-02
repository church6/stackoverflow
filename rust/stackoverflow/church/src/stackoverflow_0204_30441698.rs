// @filename   : stackoverflow_0204_30441698.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30441698
// @title      : How do I create a map from a list in a functional way?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::collections::HashMap;

        fn example() {
            let tuples = [("one", 1), ("two", 2), ("three", 3)];
            let m: HashMap<_, _> = tuples.into_iter().collect();
            println!("{:?}", m);
        }

        //

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        use std::collections::HashMap;

        fn example() {
            let m: HashMap<_, _> = HashMap::from_iter([("one", 1), ("two", 2), ("three", 3)]);
            println!("{:?}", m);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        use std::collections::HashMap;
        fn example() {
            let tuples = vec![("one", 1), ("two", 2), ("one", 3)];
            let mut m = HashMap::new();
            for (k, v) in tuples {
                m.entry(k).or_insert_with(Vec::new).push(v)
            }
            println!("{:?}", m);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code4 {
        use std::{cmp::Eq, collections::HashMap, hash::Hash, iter::FromIterator};

        struct MyCoolType<K: Eq + Hash, V>(HashMap<K, Vec<V>>);

        impl<K: Eq + Hash, V> FromIterator<(K, V)> for MyCoolType<K, V> {
            fn from_iter<I>(tuples: I) -> Self
            where
                I: IntoIterator<Item = (K, V)>,
            {
                let mut m = HashMap::new();
                for (k, v) in tuples {
                    m.entry(k).or_insert_with(Vec::new).push(v)
                }
                Self(m)
            }
        }

        fn example() {
            let tuples = vec![("one", 1), ("two", 2), ("one", 3)];
            let MyCoolType(m) = tuples.into_iter().collect();
            println!("{:?}", m);
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
        use std::collections::HashMap;
        pub fn test() {
            // add your code here
            let map = HashMap::from([("one", 1), ("two", 2)]);
            println!("{:?}", map);
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
        use std::collections::HashMap;

        fn main() {
            let m: HashMap<&str, u16> = [("year", 2019), ("month", 12)].iter().cloned().collect();
            println!("{:?}", m);
        }

        pub fn test() {
            // add your code here
        }
    }
    mod code2 {
        // Or you can do a Trait:
        use std::collections::HashMap;

        trait Hash {
            fn to_map(&self) -> HashMap<&str, u16>;
        }

        impl Hash for [(&str, u16)] {
            fn to_map(&self) -> HashMap<&str, u16> {
                self.iter().cloned().collect()
            }
        }

        fn example() {
            let m = [("year", 2019), ("month", 12)].to_map();
            println!("{:?}", m)
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
        //code1::test();
        //code2::test();
        //code3::test();
    }
}
pub fn test() {
    _enter!();
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
