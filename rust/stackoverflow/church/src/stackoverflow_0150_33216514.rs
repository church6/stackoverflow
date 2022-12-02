// @filename   : stackoverflow_0150_33216514.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/33216514
// @title      : How do I convert a Vec<String> to Vec<&str>?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        fn example2() {
            let v: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            let v2: Vec<&str> = v.iter().map(|s| &**s).collect();
            println!("{:?}", v2);
        }

        fn example3() {
            let v: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            let v3: Vec<&str> = v.iter().map(std::ops::Deref::deref).collect();
            println!("{:?}", v3);
        }

        fn example4() {
            let v: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            let v4: Vec<&str> = v.iter().map(|s| s as &str).collect();
            println!("{:?}", v4);
        }

        fn example5() {
            let v: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            let v5: Vec<&str> = v.iter().map(|s| &s[..]).collect();
            println!("{:?}", v5);
        }

        fn example6() {
            let v: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            let v6: Vec<&str> = v
                .iter()
                .map(|s| {
                    let s: &str = s;
                    s
                })
                .collect();
            println!("{:?}", v6);
        }

        fn example7() {
            let v: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            let v7: Vec<&str> = v.iter().map(|s| s.as_ref()).collect();
            println!("{:?}", v7);
        }

        fn example8() {
            let v: Vec<String> = vec!["a".to_string(), "b".to_string(), "c".to_string()];
            let v8: Vec<&str> = v.iter().map(AsRef::as_ref).collect();
            println!("{:?}", v8);
        }

        pub fn test() {
            // add your code here
            example2();
            example3();
            example4();
            example5();
            example6();
            example7();
            example8();
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
