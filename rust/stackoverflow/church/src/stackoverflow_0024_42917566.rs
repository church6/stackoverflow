// @filename   : stackoverflow_0024_42917566.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/42917566
// @title      : What is this question mark operator about?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[derive(Debug)]
        enum Error {
            Odd,
        }
        fn halves_if_even(i: i32) -> Result<i32, Error> {
            if i % 2 == 0 {
                Ok(i / 2)
            } else {
                Err(Error::Odd /* something */)
            }
        }

        fn do_the_thing(i: i32) -> Result<i32, Error> {
            let i = match halves_if_even(i) {
                Ok(i) => i,
                Err(e) => return Err(e),
            };

            // use `i`
            println!("{}", i);
            Ok(i)
        }
        pub fn test() {
            // add your code here
            match do_the_thing(2i32) {
                Ok(i) => println!("{}", i),
                Err(e) => println!("{:?}", e),
            };
        }
    }
    mod code2 {
        #[derive(Debug)]
        enum Error {
            Odd,
        }
        fn halves_if_even(i: i32) -> Result<i32, Error> {
            if i % 2 == 0 {
                Ok(i / 2)
            } else {
                Err(Error::Odd /* something */)
            }
        }
        fn do_the_thing(i: i32) -> Result<i32, Error> {
            let i = halves_if_even(i)?;

            // use `i`
            println!("{}", i);
            Ok(i)
        }
        pub fn test() {
            // add your code here
            match do_the_thing(2i32) {
                Ok(i) => println!("{}", i),
                Err(e) => println!("{:?}", e),
            };
        }
    }
    mod code3 {
        #[derive(Debug)]
        enum Error {
            Odd,
        }
        fn halves_if_even(i: i32) -> Option<i32> {
            if i % 2 == 0 {
                Some(i / 2)
            } else {
                Some(0i32) //None
            }
        }
        fn do_the_thing(i: i32) -> Option<i32> {
            let i = halves_if_even(i)?;

            // use `i`
            println!("{}", i);
            Some(i)
        }

        pub fn test() {
            // add your code here
            match do_the_thing(2i32) {
                Some(i) => println!("{}", i),
                None => println!("None"),
            };
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
