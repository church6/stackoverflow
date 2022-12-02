// @filename   : stackoverflow_0152_27831944.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27831944
// @title      : How do I store a closure in a struct in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // Unboxed closure
        // Use a generic type:

        struct Foo<F>
        where
            F: Fn(usize) -> usize,
        {
            pub foo_obj: F,
        }

        impl<F> Foo<F>
        where
            F: Fn(usize) -> usize,
        {
            fn new(foo_obj: F) -> Self {
                Self { foo_obj }
            }
        }

        fn example() {
            let foo_obj = Foo { foo_obj: |a| a + 1 };
            let rc = (foo_obj.foo_obj)(42);
            println!("{}", rc);

            let rc = (Foo::new(|a| a + 1).foo_obj)(42);
            println!("{}", rc);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        // Boxed trait object
        struct Foo {
            pub foo_obj: Box<dyn Fn(usize) -> usize>,
        }

        impl Foo {
            fn new(foo_obj: impl Fn(usize) -> usize + 'static) -> Self {
                Self {
                    foo_obj: Box::new(foo_obj),
                }
            }
        }

        fn example() {
            let foo_obj = Foo {
                foo_obj: Box::new(|a| a + 1),
            };
            let rc = (foo_obj.foo_obj)(42);
            println!("{}", rc);

            let rc = (Foo::new(|a| a + 1).foo_obj)(42);
            println!("{}", rc);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        // Trait object reference
        struct Foo<'a> {
            pub foo_obj: &'a dyn Fn(usize) -> usize,
        }

        impl<'a> Foo<'a> {
            fn new(foo_obj: &'a dyn Fn(usize) -> usize) -> Self {
                Self { foo_obj }
            }
        }

        fn example() {
            let foo_obj = Foo {
                foo_obj: &|a| a + 1,
            };
            let rc = (foo_obj.foo_obj)(42);
            println!("{}", rc);

            let rc = (Foo::new(&|a| a + 1).foo_obj)(42);
            println!("{}", rc);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code4 {
        // Function pointer
        struct Foo {
            pub foo_obj: fn(usize) -> usize,
        }

        impl Foo {
            fn new(foo_obj: fn(usize) -> usize) -> Self {
                Self { foo_obj }
            }
        }

        fn example() {
            let foo_obj = Foo { foo_obj: |a| a + 1 };
            let rc = (foo_obj.foo_obj)(42);
            println!("{}", rc);

            let rc = (Foo::new(|a| a + 1).foo_obj)(42);
            println!("{}", rc);
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    pub fn test() {
        code1::test();
        code2::test();
        code3::test();
        code4::test();
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
