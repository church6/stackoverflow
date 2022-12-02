// @filename   : stackoverflow_0131_26212397.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/26212397
// @title      : References to traits in structs

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        // With references
        trait Foo {}
        #[derive(Debug)]
        struct MyFoo;

        impl Foo for MyFoo {}

        struct Bar<'a> {
            foo: &'a (dyn Foo + 'a),
        }

        impl<'a> Bar<'a> {
            fn new(the_foo: &'a dyn Foo) -> Bar<'a> {
                Bar { foo: the_foo }
            }

            fn get_foo(&'a self) -> &'a dyn Foo {
                self.foo
            }
        }

        fn example() {
            let myfoo = MyFoo {};
            println!("{:?}", myfoo);
            let _mybar = Bar::new(&myfoo as &dyn Foo);
        }
        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        // With Boxes
        trait Foo {}
        #[derive(Debug)]
        struct MyFoo;

        impl Foo for MyFoo {}

        struct Bar<'a> {
            foo: Box<dyn Foo + 'a>,
        }

        impl<'a> Bar<'a> {
            fn new(the_foo: Box<dyn Foo + 'a>) -> Bar<'a> {
                Bar { foo: the_foo }
            }

            fn get_foo(&'a self) -> &'a dyn Foo {
                &*self.foo
            }
        }

        fn example() {
            let myfoo = MyFoo {};
            println!("{:?}", myfoo);
            //let mybar = Bar::new(box MyFoo as Box<dyn Foo>);
            let _mybar = Bar::new(Box::new(myfoo));
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        // 'static version would be :
        trait Foo {}
        #[derive(Debug)]
        struct MyFoo;

        impl Foo for MyFoo {}

        struct Bar {
            foo: Box<dyn Foo + 'static>,
        }

        impl Bar {
            fn new(the_foo: Box<dyn Foo + 'static>) -> Bar {
                Bar { foo: the_foo }
            }

            #[allow(clippy::needless_lifetimes)]
            fn get_foo<'a>(&'a self) -> &'a dyn Foo {
                &*self.foo
            }
        }

        fn example() {
            let myfoo = MyFoo {};
            println!("{:?}", myfoo);
            //let mybar = Bar::new(box MyFoo as Box<dyn Foo>);
            let _mybar = Bar::new(Box::new(myfoo));
            let _x = _mybar.get_foo();
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
