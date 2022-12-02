// @filename   : stackoverflow_0050_41081240.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/41081240
// @title      : Idiomatic callbacks in Rust

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        type Callback = fn();

        struct Processor {
            callback: Callback,
        }

        impl Processor {
            fn set_callback(&mut self, c: Callback) {
                self.callback = c;
            }

            fn process_events(&self) {
                (self.callback)();
            }
        }

        fn simple_callback() {
            println!("hello world!");
        }

        pub fn test() {
            // add your code here
            let p = Processor {
                callback: simple_callback,
            };
            p.process_events(); // hello world!
        }
    }
    mod code2 {
        struct Processor<CB>
        where
            CB: FnMut(),
        {
            callback: CB,
        }

        impl<CB> Processor<CB>
        where
            CB: FnMut(),
        {
            fn set_callback(&mut self, c: CB) {
                self.callback = c;
            }

            fn process_events(&mut self) {
                (self.callback)();
            }
        }
        pub fn test() {
            // add your code here
            let s = "world!".to_string();
            let callback = || println!("hello {}", s);
            let mut p = Processor { callback };
            p.process_events();
        }
    }
    mod code3 {
        struct Processor {
            callback: Box<dyn FnMut()>,
        }

        impl Processor {
            fn set_callback(&mut self, c: impl FnMut() + 'static) {
                self.callback = Box::new(c);
            }

            fn process_events(&mut self) {
                (self.callback)();
            }
        }

        fn simple_callback() {
            println!("hello");
        }
        pub fn test() {
            // add your code here
            let mut p = Processor {
                callback: Box::new(simple_callback),
            };
            p.process_events();
            let s = "world!".to_string();
            let callback2 = move || println!("hello {}", s);
            p.set_callback(callback2);
            p.process_events();
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
