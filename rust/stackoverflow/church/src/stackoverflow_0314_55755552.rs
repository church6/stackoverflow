// @filename   : stackoverflow_0314_55755552.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/55755552
// @title      : What is the Rust equivalent to a try-catch statement?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[derive(Debug)]
        enum MyError {
            Oops,
        }

        // There is no try catch statement in Rust. The closest approach is the ? operator.

        //However, you do not have to create a function and a match statement to resolve it in the end. You can define a closure in your scope and use ? operator inside the closure. Then throws are held in the closure return value and you can catch this wherever you want like following:
        fn do_step_1() -> Result<(), MyError> {
            Ok(())
        }
        fn do_step_2() -> Result<(), MyError> {
            Ok(())
        }
        fn do_step_3() -> Result<(), MyError> {
            //Ok(())
            Err(MyError::Oops)
        }
        fn example() {
            let do_steps = || -> Result<(), MyError> {
                do_step_1()?;
                do_step_2()?;
                do_step_3()?;
                Ok(())
            };

            if let Err(_err) = do_steps() {
                println!("Failed to perform necessary steps: {:?}", _err);
            }
        }

        pub fn test() {
            // add your code here
            example();
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
        #[derive(Debug)]
        enum MyError {
            Oops,
        }
        fn do_step_1() -> Result<(), MyError> {
            Ok(())
        }
        fn do_step_2() -> Result<(), MyError> {
            Ok(())
        }
        fn do_step_3() -> Result<(), MyError> {
            //Ok(())
            Err(MyError::Oops)
        }

        // Results in Rust can be chained using and_then. So you can do this:
        fn example() {
            // FIXME
            /*if let Err(e) = do_step_1().and_then(do_step_2).and_then(do_step_3) {
                println!("Failed to perform necessary steps: {:?}", e);
            }*/
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        #[derive(Debug)]
        enum MyError {
            Oops,
        }
        fn do_step_1() -> Result<(), MyError> {
            Ok(())
        }
        fn do_step_2() -> Result<(), MyError> {
            Ok(())
        }
        fn do_step_3() -> Result<(), MyError> {
            //Ok(())
            Err(MyError::Oops)
        }
        //or if you want a more compact syntax, you can do it with a macro:
        macro_rules! attempt { // `try` is a reserved keyword
   (@recurse ($a:expr) { } catch ($e:ident) $b:block) => {
      if let Err ($e) = $a $b
   };
   (@recurse ($a:expr) { $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      attempt!{@recurse ($a.and_then (|_| $e)) { $($tail)* } $($handler)*}
   };
   ({ $e:expr; $($tail:tt)* } $($handler:tt)*) => {
      attempt!{@recurse ($e) { $($tail)* } $($handler)* }
   };
}
        fn example() {
            attempt! {{
               do_step_1();
               do_step_2();
               do_step_3();
            } catch (e) {
               println!("Failed to perform necessary steps: {:?}", e);
            }}
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
