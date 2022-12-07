// @filename   : stackoverflow_0305_46388386.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/46388386
// @title      : What exactly does '#[derive(Debug)]' mean in Rust?

#[allow(dead_code)]
mod answer1 {
    /*

    You can see what the compiler did by executing cargo +nightly rustc -- -Zunstable-options --pretty=expanded. In your example, the compiler will add something like

    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl <'a> ::std::fmt::Debug for Person<'a> {
        fn fmt(&self, __arg_0: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
            match *self {
                Person { name: ref __self_0_0, age: ref __self_0_1 } => {
                    let mut builder = __arg_0.debug_struct("Person");
                    let _ = builder.field("name", &&(*__self_0_0));
                    let _ = builder.field("age", &&(*__self_0_1));
                    builder.finish()
                }
            }

    */

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
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
