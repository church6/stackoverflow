// @filename   : stackoverflow_0033_30177395.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/30177395
// @title      : When does a closure implement Fn, FnMut and FnOnce?

#[allow(dead_code)]
mod answer1 {
    /*
    FnOnce (self) are functions that can be called once
    FnMut (&mut self) are functions that can be called if they have &mut access to their environment
    Fn (&self) are functions that can be called if they only have & access to their environment
    */

    /*
    A closure |...| ... will automatically implement as many of those as it can.
    All closures implement FnOnce: a closure that can't be called once doesn't deserve the name. Note that if a closure only implements FnOnce, it can be called only once.
    Closures that don't move out of their captures implement FnMut, allowing them to be called more than once (if there is unaliased access to the function object).
    Closures that don't need unique/mutable access to their captures implement Fn, allowing them to be called essentially everywhere.

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
