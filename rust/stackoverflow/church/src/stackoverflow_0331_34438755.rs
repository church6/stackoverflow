// @filename   : stackoverflow_0331_34438755.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/34438755
// @title      : Why would I implement methods on a trait instead of as part of the trait?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use std::any::type_name;

        trait Foo {
            fn foo_in_trait(&self)
            where
                Self: 'static,
            {
                println!("{}", type_name::<Self>());
            }
        }

        impl dyn Foo {
            fn foo_in_impl(&self) {
                println!("{}", type_name::<Self>());
            }
        }

        impl Foo for u8 {}
        impl Foo for u16 {}

        fn example() {
            let x = Box::new(42u8) as Box<dyn Foo>;
            x.foo_in_trait();
            x.foo_in_impl();

            let x = Box::new(42u16) as Box<dyn Foo>;
            x.foo_in_trait();
            x.foo_in_impl();
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
