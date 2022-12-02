// @filename   : stackoverflow_0078_21371534.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/21371534
// @title      : In Rust, is there a way to iterate through the values of an enum?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        use strum::IntoEnumIterator;
        use strum_macros::EnumIter;

        #[derive(Debug, EnumIter)]
        enum Direction {
            North,
            South,
            East,
            West,
        }
        pub fn test() {
            // add your code here
            for direction in Direction::iter() {
                println!("{:?}", direction);
            }
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
        code2::test();
        //code3::test();
    }
}
#[allow(dead_code)]
mod answer2 {
    mod code1 {
        use self::Direction::*;
        use std::slice::Iter;

        #[derive(Debug)]
        pub enum Direction {
            North,
            South,
            East,
            West,
        }

        impl Direction {
            pub fn iterator() -> Iter<'static, Direction> {
                static DIRECTIONS: [Direction; 4] = [North, South, East, West];
                DIRECTIONS.iter()
            }
        }
        pub fn test() {
            // add your code here
            for dir in Direction::iterator() {
                println!("{:?}", dir);
            }
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
mod answer3 {
    mod code1 {
        #[derive(Debug, Copy, Clone)]
        enum Direction {
            North,
            South,
            East,
            West,
        }
        impl Direction {
            const VALUES: [Self; 4] = [Self::North, Self::South, Self::East, Self::West];
        }
        //#[allow(clippy::unnecessary_to_owned)]
        pub fn test() {
            // add your code here
            // Direction::VALUES.iter().copied()
            for direction in Direction::VALUES.iter() {
                println!("{:?}", direction);
                // todo!();
            }
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
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    _leave!();
}
