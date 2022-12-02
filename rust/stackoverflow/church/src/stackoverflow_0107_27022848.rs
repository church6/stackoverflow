// @filename   : stackoverflow_0107_27022848.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/27022848
// @title      : How I can mutate a struct's field from a method?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        struct Point {
            x: i32,
            y: i32,
        }

        impl Point {
            fn up(&mut self) {
                // ^^^ Here
                self.y += 1;
            }
        }
        pub fn test() {
            // add your code here
            let mut p = Point { x: 0, y: 0 };
            println!("{0},{1}", p.x, p.y);
            //  ^^^ And here
            p.up();
            println!("{0},{1}", p.x, p.y);
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
        use std::cell::Cell;

        struct Point {
            x: i32,
            y: Cell<i32>,
        }

        impl Point {
            fn up(&self) {
                self.y.set(self.y.get() + 1);
            }
        }
        pub fn test() {
            // add your code here
            let p = Point {
                x: 0,
                y: Cell::new(0),
            };
            p.up();
            println!("y: {:?}", p.y);
        }
    }
    mod code2 {
        struct Point {
            x: i32,
            y: i32,
        }

        // #![feature(cell_update)]
        /*
        impl Point {
            fn up(&self) {
                self.y.update(|x| x + 1);
            }
        }*/
        pub fn test() {
            // add your code here
            let p = Point { x: 0, y: 0 };
            println!("{0},{1}", p.x, p.y);
            // p.up();
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
