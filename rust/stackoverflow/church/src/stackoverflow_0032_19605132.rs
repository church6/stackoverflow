// @filename   : stackoverflow_0032_19605132.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/19605132
// @title      : Is it possible to use global variables in Rust?

#[allow(dead_code)]
mod answer1 {
    // It's possible, but heap allocation is not allowed directly. Heap allocation is performed at runtime.
    mod code1 {
        static SOME_INT: i32 = 5;
        #[allow(clippy::redundant_static_lifetimes)]
        static SOME_STR: &'static str = "A static string";
        static SOME_STRUCT: MyStruct = MyStruct {
            number: 10,
            string: "Some string",
        };

        //static mut db: Option<sqlite::Connection> = None;

        struct MyStruct {
            number: i32,
            string: &'static str,
        }

        pub fn test() {
            // add your code here

            println!("{}", SOME_INT);
            println!("{}", SOME_STR);
            println!("{}", SOME_STRUCT.number);
            println!("{}", SOME_STRUCT.string);

            //unsafe {
            //db = Some(open_database());
            //}
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
