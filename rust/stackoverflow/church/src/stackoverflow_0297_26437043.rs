// @filename   : stackoverflow_0297_26437043.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/26437043
// @title      : Why does Rust have struct and enum?

#[allow(dead_code)]
mod answer1 {
    /*

    First of all, you are correct that semantically enum is strictly superior to the struct as to what it can represent, and therefore struct is somewhat redundant.

    However, there are other elements at play here.

    ease of use: the values within an enum can only be accessed (directly) through matching; contrast with the ease of use of accessing a struct field. You could write accessors for each and every field, but that is really cumbersome.

    distinction: an enum is a tagged union, a struct has a fixed-layout; we (programmers) generally like to put labels on things, and therefore giving different names to different functionality can be appreciated.

    As I see it, struct is therefore syntactic sugar. I usually prefer lean and mean, but a bit of sugar can go a long way in increasing what can be represented tersely.

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
