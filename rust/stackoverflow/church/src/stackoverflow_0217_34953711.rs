// @filename   : stackoverflow_0217_34953711.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/34953711
// @title      : Unwrap inner type when enum variant is known

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[derive(Debug, Clone, Copy)]
        struct Dog(i32);
        #[derive(Debug, Clone, Copy)]
        struct Cat(u8);

        enum Animal {
            Dog(Dog),
            Cat(Cat),
        }

        impl Animal {
            fn cat(self) -> Cat {
                if let Animal::Cat(c) = self {
                    c
                } else {
                    panic!("Not a cat")
                }
            }

            fn dog(self) -> Dog {
                if let Animal::Dog(d) = self {
                    d
                } else {
                    panic!("Not a dog")
                }
            }
        }

        // Or better an impl on `Cat` ?
        fn count_legs_of_cat0(c: Cat) -> u8 {
            c.0
        }

        fn count_legs_of_cat1(animal: Animal) -> u8 {
            if let Animal::Cat(c) = animal {
                c.0
            } else {
                unreachable!()
            }
        }

        fn example() {
            let cat = Cat(4u8);
            //let animal = Animal::Cat(cat.clone());
            assert_eq!(4u8, count_legs_of_cat0(cat));
            let animal = Animal::Cat(cat);
            assert_eq!(4u8, count_legs_of_cat1(animal));
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code2 {
        #[derive(Debug, Clone, Copy)]
        struct Dog(i32);
        #[derive(Debug, Clone, Copy)]
        struct Cat(u8);

        enum Animal {
            Dog(Dog),
            Cat(Cat),
        }

        impl Animal {
            fn cat(self) -> Option<Cat> {
                if let Animal::Cat(c) = self {
                    Some(c)
                } else {
                    None
                }
            }

            fn dog(self) -> Option<Dog> {
                if let Animal::Dog(d) = self {
                    Some(d)
                } else {
                    None
                }
            }
        }
        // Or better an impl on `Cat` ?
        fn count_legs_of_cat0(c: Cat) -> u8 {
            c.0
        }

        fn count_legs_of_cat1(animal: Animal) -> u8 {
            if let Animal::Cat(c) = animal {
                c.0
            } else {
                unreachable!()
            }
        }

        fn example() {
            let cat = Cat(4u8);
            //let animal = Animal::Cat(cat.clone());
            assert_eq!(4u8, count_legs_of_cat0(cat));
            let animal = Animal::Cat(cat);
            assert_eq!(4u8, count_legs_of_cat1(animal));
        }

        pub fn test() {
            // add your code here
            example();
        }
    }
    mod code3 {
        #[derive(Debug, Clone, Copy)]
        struct Dog(i32);
        #[derive(Debug, Clone, Copy)]
        struct Cat(u8);

        enum Animal {
            Dog(Dog),
            Cat(Cat),
        }

        impl Animal {
            fn cat(self) -> Option<Cat> {
                match self {
                    Animal::Cat(c) => Some(c),
                    _ => None,
                }
            }

            fn dog(self) -> Option<Dog> {
                match self {
                    Animal::Dog(d) => Some(d),
                    _ => None,
                }
            }
        }

        // Or better an impl on `Cat` ?
        fn count_legs_of_cat0(c: Cat) -> u8 {
            c.0
        }

        fn count_legs_of_cat1(animal: Animal) -> u8 {
            if let Animal::Cat(c) = animal {
                c.0
            } else {
                unreachable!()
            }
        }

        fn example() {
            let cat = Cat(4u8);
            //let animal = Animal::Cat(cat.clone());
            assert_eq!(4u8, count_legs_of_cat0(cat));
            let animal = Animal::Cat(cat);
            assert_eq!(4u8, count_legs_of_cat1(animal));
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
