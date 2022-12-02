// @filename   : stackoverflow_0268_45282970.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/45282970
// @title      : Does Rust have an equivalent to Python's list comprehension syntax?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // l = [x for x in range(1, 10) if x % 2 == 0]
        // [2, 4, 6, 8]

        fn example() {
            let v1 = (0u32..9)
                .filter(|x| x % 2 == 0)
                .map(|x| x.pow(2))
                .collect::<Vec<_>>();
            let v2 = (1..10).filter(|x| x % 2 == 0).collect::<Vec<u32>>();

            println!("{:?}", v1); // [0, 4, 16, 36, 64]
            println!("{:?}", v2); // [2, 4, 6, 8]
            assert_eq!(vec![0, 4, 16, 36, 64], v1);
            assert_eq!(vec![2, 4, 6, 8], v2);
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
