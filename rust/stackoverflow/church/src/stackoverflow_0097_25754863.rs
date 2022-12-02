// @filename   : stackoverflow_0097_25754863.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/25754863
// @title      : How to create a Rust struct with string members?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        struct Foo {
            bar: String,
            baz: &'static str,
        }
        pub fn test() {
            // add your code here
            let _foo = Foo {
                bar: "bar".to_string(),
                baz: "baz",
            };
            println!("{}, {}", _foo.bar, _foo.baz);
        }
    }
    mod code2 {
        use std::borrow::Cow;

        struct Foo<'a> {
            baz: Cow<'a, str>,
        }

        pub fn test() {
            // add your code here

            let foo1 = Foo {
                baz: Cow::Borrowed("baz"),
            };
            let foo2 = Foo {
                baz: Cow::Owned("baz".to_string()),
            };
            println!("{}, {}", foo1.baz, foo2.baz);
        }
    }
    mod code3 {
        pub fn test() {
            // add your code here
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
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
