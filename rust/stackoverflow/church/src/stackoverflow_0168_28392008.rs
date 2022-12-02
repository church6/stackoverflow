// @filename   : stackoverflow_0168_28392008.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28392008
// @title      : Is there a more concise or declarative way to initialize a HashMap? [duplicate]

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::collections::HashMap;

        fn example1() {
            let counts = "ACGT"
                .chars()
                .map(|c| (c, 0_i32))
                .collect::<HashMap<_, _>>();
            println!("{:?}", counts);
        }

        fn example2() {
            let mut counts: HashMap<char, i32> = HashMap::new();
            for c in "ACGT".chars() {
                // counts.insert(c, 0).unwrap();
                counts.entry(c).or_insert(0);
            }
            println!("{:?}", counts);
        }

        macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

        fn example3() {
            let counts = hashmap!['A' => 0, 'C' => 0, 'G' => 0, 'T' => 0];
            println!("{:?}", counts);
        }
        pub fn test() {
            // add your code here
            example1();
            example2();
            example3();
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

        fn example1() {
            let timber_resources: HashMap<&str, i32> =
                [("Norway", 100), ("Denmark", 50), ("Iceland", 10)]
                    .iter()
                    .cloned()
                    .collect();
            // use the values stored in map
            println!("{:?}", timber_resources);
        }

        fn example2() {
            let vikings = HashMap::from([("Norway", 25), ("Denmark", 24), ("Iceland", 12)]);
            println!("{:?}", vikings);
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
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

        fn example() {
            let m = HashMap::from([('A', 0), ('C', 0), ('G', 0), ('T', 0)]);
            println!("{:?}", m);
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