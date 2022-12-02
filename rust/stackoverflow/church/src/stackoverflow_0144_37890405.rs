// @filename   : stackoverflow_0144_37890405.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/37890405
// @title      : Is there a way to simplify converting an Option into a Result without a macro?

#[allow(dead_code)]
mod answer1 {
    // The ok_or and ok_or_else methods convert Options to Results, and the ? operator automates the boilerplate associated with early Err returns.
    mod code1 {
        struct Boo {}

        fn new() -> Result<Boo, String> {
            //let item1 = section.get("item1").ok_or("no item1")?;
            //let item2 = section.get("item2").ok_or("no item2")?;
            // whatever processing...
            //Ok(final_result)
            let b = Boo {};
            Ok(b)
        }

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
