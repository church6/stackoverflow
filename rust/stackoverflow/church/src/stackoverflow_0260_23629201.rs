// @filename   : stackoverflow_0260_23629201.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/23629201
// @title      : Are nested structs supported in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[derive(Debug)]
        struct Inner {
            i: i32,
        }

        #[derive(Debug)]
        struct Outer {
            o: i32,
            inner: Inner,
        }

        pub fn test() {
            // add your code here
            let obj = Outer {
                o: 10,
                inner: Inner { i: 9 },
            };
            assert!(10i32 == obj.o);
            assert!(9i32 == obj.inner.i);
            println!("{}", obj.o);
            println!("{}", obj.inner.i);
            println!("{:?}", obj);
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
