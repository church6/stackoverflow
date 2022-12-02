// @filename   : stackoverflow_0094_26611664.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/26611664
// @title      : What is the r#""# operator in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[allow(clippy::eq_op)]
        pub fn test() {
            // add your code here
            let var1 = "test1";
            println!("{var1}");
            let json = r#"{"type": "type1", "type2": var1}"#;
            println!("{}", json); // => {"type2": "type1", "type2": var1}

            assert!("foo" == r"foo"); // foo
            assert!("\"foo\"" == r#""foo""#); // "foo"

            assert!("foo #\"# bar" == r##"foo #"# bar"##); // foo #"# bar

            assert!("\x52" == "R"); // R
            assert!("\x52" == r"R");
            assert!("\\x52" == r"\x52"); // \x52
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
