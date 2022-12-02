// @filename   : stackoverflow_0285_42722169.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/42722169
// @title      : Generate pretty (indented) JSON with serde

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use serde_json::json;

        fn example() {
            let obj = json!({"foo":1,"bar":2});
            println!("{}", serde_json::to_string_pretty(&obj).unwrap());
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        use serde::Serialize;
        use serde_json::json;

        fn example() {
            let obj = json!({"foo":1,"bar":2});

            let buf = Vec::new();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
            let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
            obj.serialize(&mut ser).unwrap();
            println!("{}", String::from_utf8(ser.into_inner()).unwrap());
        }
        pub fn test() {
            // add your code here
            example();
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
