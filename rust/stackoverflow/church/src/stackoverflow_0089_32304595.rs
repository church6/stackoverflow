// @filename   : stackoverflow_0089_32304595.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/32304595
// @title      : What's the difference between self and Self?

#[allow(dead_code)]
mod answer1 {
    /*
    Self is the type of the current object.
    It may appear either in a trait or an impl, but appears most often in trait where it is a stand-in for whatever type will end up implementing the trait (which is unknown when defining the trait):
    */
    mod code1 {
        struct MyType {
            val: i32,
        }
        impl MyType {
            fn doit(&self, a: u32) {
                println!("doit    ={a}");
            }
            fn another(this: &Self, a: u32) {
                this.doit(a);
                println!("another = {a}");
            }
        }
        pub fn test() {
            // add your code here

            let m = MyType { val: 100 };

            // Both can be used as an associated function
            MyType::doit(&m, 1);
            MyType::another(&m, 2);

            // But only `doit` can be used in method position
            m.doit(3); // OK: `m` is automatically borrowed
                       // this is an associated function, not a method
                       // m.another(4); // ERROR: no method named `another`
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
