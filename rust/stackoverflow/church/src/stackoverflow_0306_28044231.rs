// @filename   : stackoverflow_0306_28044231.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28044231
// @title      : What does "Sized is not implemented" mean?

#[allow(dead_code)]
mod answer1 {
    mod code1 {

        use std::io::{Result, Write};
        // use std::io::stdio;

        fn example() {
            // FIXME
            // let h = |&: w: &mut dyn Write| -> Result<()> { writeln!(w, "foo") };
            // let _ = h.handle(&mut stdio::stdout());
        }

        trait Handler<W: ?Sized>
        where
            W: Write,
        {
            fn handle(&self, _: &mut W) -> Result<()>;
        }

        impl<W: ?Sized, F> Handler<W> for F
        where
            W: Write,
            F: Fn(&mut W) -> Result<()>,
        {
            fn handle(&self, w: &mut W) -> Result<()> {
                (*self)(w)
            }
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        // And for the Rust 1.0 code:

        use std::io::{self, Write};

        fn example() {
            handle(&mut io::stdout()).unwrap();
        }

        fn handle(w: &mut dyn Write) -> io::Result<()> {
            handler(w)
        }

        fn handler<W: ?Sized>(w: &mut W) -> io::Result<()>
        where
            W: Write,
        {
            writeln!(w, "foo")
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
