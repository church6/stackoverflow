// @filename   : stackoverflow_0120_33687447.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/33687447
// @title      : How to get a reference to a concrete type from a trait object?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::any::Any;

        trait A {
            fn as_any(&self) -> &dyn Any;
        }

        struct B;

        impl A for B {
            fn as_any(&self) -> &dyn Any {
                self
            }
        }

        fn example1() {
            let a: Box<dyn A> = Box::new(B);
            // The indirection through `as_any` is because using `downcast_ref`
            // on `Box<A>` *directly* only lets us downcast back to `&A` again.
            // The method ensures we get an `Any` vtable that lets us downcast
            // back to the original, concrete type.
            let _b: &B = match a.as_any().downcast_ref::<B>() {
                Some(b) => b,
                None => panic!("&a isn't a B!"),
            };
        }

        fn example2() {
            let a: Box<dyn Any> = Box::new(B);
            let _: &B = match a.downcast_ref::<B>() {
                Some(b) => b,
                None => panic!("&a isn't a B!"),
            };
        }

        pub fn test() {
            // add your code here
            example1();
            example2();
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
