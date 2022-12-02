// @filename   : stackoverflow_0104_30026893.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30026893
// @title      : Using map with Vectors

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn f(n: &i32) -> i32 {
            n + 1
        }
        pub fn test() {
            // add your code here
            {
                let u = vec![1, 2, 3];
                println!("{:?}", u);
                let v: Vec<_> = u.iter().map(f).collect();
                println!("{:?}", v);
            }
            {
                let u = vec![1, 2, 3];
                println!("{:?}", u);
                let v = u.iter().map(|&x| x + 1).collect::<Vec<_>>();
                println!("{:?}", v);
            }
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
