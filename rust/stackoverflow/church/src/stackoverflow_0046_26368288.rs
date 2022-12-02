// @filename   : stackoverflow_0046_26368288.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/26368288
// @title      : How do I stop iteration and return an error when Iterator::map returns a Result::Err?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[derive(Debug)]
        struct Item;
        type Id = String;

        fn find(id: &Id) -> Result<Item, String> {
            Err(format!("Not found: {:?}", id))
        }
        pub fn test() {
            // add your code here
            let s = |s: &str| s.to_string();
            let ids = vec![s("1"), s("2"), s("3")];

            let items: Result<Vec<_>, _> = ids.iter().map(find).collect();
            println!("Result: {:?}", items);
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
        // Iterator consumer: try_for_each
        use std::{fs, io};
        pub fn fs_test() -> io::Result<()> {
            fs::read_dir("/tmp")?.try_for_each(|e| -> io::Result<()> {
                println!("{}", e?.path().display());
                Ok(())
            })?;
            // ...
            Ok(())
        }
        pub fn test() {
            // add your code here
            fs_test().unwrap();
        }
    }
    mod code2 {
        // Iterator adapter: scan
        use std::{fs, io};

        fn fs_test() -> io::Result<()> {
            fs::read_dir("/")?
                .take_while(Result::is_ok)
                .map(Result::unwrap)
                .for_each(|e| println!("{}", e.path().display()));
            // ...
            Ok(())
        }
        pub fn test() {
            // add your code here
            fs_test().unwrap();
        }
    }
    mod code3 {
        use std::{fs, io};
        fn fs_test() -> io::Result<()> {
            let mut err = Ok(());
            fs::read_dir("/")?
                .scan(&mut err, |err, res| match res {
                    Ok(o) => Some(o),
                    Err(e) => {
                        **err = Err(e);
                        None
                    }
                })
                .for_each(|e| println!("{}", e.path().display()));
            err?;
            // ...
            Ok(())
        }
        pub fn test() {
            // add your code here
            fs_test().unwrap();
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
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
    answer2::test();
    //answer3::test();
    _leave!();
}
