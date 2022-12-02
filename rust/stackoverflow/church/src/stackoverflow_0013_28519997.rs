// @filename   : stackoverflow_0013_28519997.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/28519997
// @title      : What are Rust's exact auto-dereferencing rules?

#[allow(clippy::needless_borrow)]
mod ask {
    struct X {
        val: i32,
    }
    impl std::ops::Deref for X {
        type Target = i32;
        fn deref(&self) -> &i32 {
            &self.val
        }
    }

    trait M {
        fn m(self);
    }
    impl M for i32 {
        fn m(self) {
            println!("i32::m()");
        }
    }
    impl M for X {
        fn m(self) {
            println!("X::m()");
        }
    }
    impl M for &X {
        fn m(self) {
            println!("&X::m()");
        }
    }
    impl M for &&X {
        fn m(self) {
            println!("&&X::m()");
        }
    }
    impl M for &&&X {
        fn m(self) {
            println!("&&&X::m()");
        }
    }

    trait RefM {
        fn refm(&self);
    }
    impl RefM for i32 {
        fn refm(&self) {
            println!("i32::refm()");
        }
    }
    impl RefM for X {
        fn refm(&self) {
            println!("X::refm()");
        }
    }
    impl RefM for &X {
        fn refm(&self) {
            println!("&X::refm()");
        }
    }
    impl RefM for &&X {
        fn refm(&self) {
            println!("&&X::refm()");
        }
    }
    impl RefM for &&&X {
        fn refm(&self) {
            println!("&&&X::refm()");
        }
    }

    struct Y {
        val: i32,
    }
    impl std::ops::Deref for Y {
        type Target = i32;
        fn deref(&self) -> &i32 {
            &self.val
        }
    }

    struct Z {
        val: Y,
    }
    impl std::ops::Deref for Z {
        type Target = Y;
        fn deref(&self) -> &Y {
            &self.val
        }
    }

    #[derive(Clone, Copy)]
    struct A;

    impl M for A {
        fn m(self) {
            println!("A::m()");
        }
    }
    impl M for &&&A {
        fn m(self) {
            println!("&&&A::m()");
        }
    }

    impl RefM for A {
        fn refm(&self) {
            println!("A::refm()");
        }
    }
    impl RefM for &&&A {
        fn refm(&self) {
            println!("&&&A::refm()");
        }
    }

    pub fn test() {
        // I'll use @ to denote left side of the dot operator
        (*X { val: 42 }).m(); // i32::m()    , Self == @
        X { val: 42 }.m(); // X::m()      , Self == @
        (&X { val: 42 }).m(); // &X::m()     , Self == @
        (&&X { val: 42 }).m(); // &&X::m()    , Self == @
        (&&&X { val: 42 }).m(); // &&&X:m()    , Self == @
        (&&&&X { val: 42 }).m(); // &&&X::m()   , Self == *@
        (&&&&&X { val: 42 }).m(); // &&&X::m()   , Self == **@
        println!("-------------------------");

        (*X { val: 42 }).refm(); // i32::refm() , Self == @
        X { val: 42 }.refm(); // X::refm()   , Self == @
        (&X { val: 42 }).refm(); // X::refm()   , Self == *@
        (&&X { val: 42 }).refm(); // &X::refm()  , Self == *@
        (&&&X { val: 42 }).refm(); // &&X::refm() , Self == *@
        (&&&&X { val: 42 }).refm(); // &&&X::refm(), Self == *@
        (&&&&&X { val: 42 }).refm(); // &&&X::refm(), Self == **@
        println!("-------------------------");

        Y { val: 42 }.refm(); // i32::refm() , Self == *@
        Z { val: Y { val: 42 } }.refm(); // i32::refm() , Self == **@
        println!("-------------------------");

        A.m(); // A::m()      , Self == @
               // without the Copy trait, (&A).m() would be a compilation error:
               // cannot move out of borrowed content
        (&A).m(); // A::m()      , Self == *@
        (&&A).m(); // &&&A::m()   , Self == &@
        (&&&A).m(); // &&&A::m()   , Self == @
        A.refm(); // A::refm()   , Self == @
        (&A).refm(); // A::refm()   , Self == *@
        (&&A).refm(); // A::refm()   , Self == **@
        (&&&A).refm(); // &&&A::refm(), Self == @
    }
}

#[allow(dead_code)]
mod answer1 {
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
    ask::test();
    //answer1::test();
    //answer2::test();
    //answer3::test();
    _leave!();
}
