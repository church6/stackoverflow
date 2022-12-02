// @filename   : stackoverflow_0055_30414424.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30414424
// @title      : How can I update a value in a mutable HashMap?

mod ask {
    use std::collections::HashMap;
    pub fn test() {
        let mut my_map = HashMap::new();
        my_map.insert("a", 1);
        my_map.insert("b", 3);

        // error[E0594]: cannot assign to data in an index of `std::collections::HashMap<&str, i32>`
        // my_map["a"] += 10;
        // I expect my_map becomes {"b": 3, "a": 11}
        println!("{:?}", my_map);
    }
}

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::collections::HashMap;
        pub fn test() {
            // add your code here

            let mut my_map = HashMap::new();
            // my_map["key"] = "value";
            println!("{:?}", my_map);

            my_map.insert("a", 1);
            my_map.insert("b", 3);

            // For now, you can use get_mut:
            *my_map.get_mut("a").unwrap() += 10;
            println!("{:?}", my_map);

            // Or the entry API:
            *my_map.entry("a").or_insert(42) += 10;
            println!("{:?}", my_map);
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
    ask::test();
    answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
