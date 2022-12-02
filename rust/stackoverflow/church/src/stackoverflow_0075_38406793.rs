// @filename   : stackoverflow_0075_38406793.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/38406793
// @title      : Why is capitalizing the first letter of a string so convoluted in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[allow(clippy::iter_nth_zero)]
        pub fn test() {
            // add your code here
            {
                let s1 = "foobar";
                let mut v: Vec<char> = s1.chars().collect();
                println!("{:?}", v);
                v[0] = v[0].to_uppercase().nth(0).unwrap();
                println!("{:?}", v);
                let s2: String = v.into_iter().collect();
                println!("{}", s2);
            }

            {
                let s1 = "foobar";
                let mut v: Vec<char> = s1.chars().collect();
                println!("{:?}", v);
                v[0] = v[0].to_uppercase().next().unwrap();
                println!("{:?}", v);
                let s2: String = v.into_iter().collect();
                println!("{}", s2);
            }
        }
    }
    mod code2 {
        fn some_kind_of_uppercase_first_letter(s: &str) -> String {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().chain(c).collect(),
            }
        }
        pub fn test() {
            // add your code here
            println!("{}", some_kind_of_uppercase_first_letter("joe"));
            println!("{}", some_kind_of_uppercase_first_letter("jill"));
            println!("{}", some_kind_of_uppercase_first_letter("von Hagen"));
            println!("{}", some_kind_of_uppercase_first_letter("ß"));
        }
    }
    mod code3 {
        fn some_kind_of_uppercase_first_letter(s: &str) -> String {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        }
        pub fn test() {
            // add your code here
            assert_eq!("Joe", some_kind_of_uppercase_first_letter("joe"));
            assert_eq!("Jill", some_kind_of_uppercase_first_letter("jill"));
            assert_eq!(
                "Von Hagen",
                some_kind_of_uppercase_first_letter("von Hagen")
            );
            assert_eq!("SS", some_kind_of_uppercase_first_letter("ß"));
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
        fn make_ascii_titlecase(s: &mut str) {
            if let Some(r) = s.get_mut(0..1) {
                r.make_ascii_uppercase();
            }
        }
        pub fn test() {
            // add your code here
            let mut s = String::from("joe");
            make_ascii_titlecase(&mut s);
            assert_eq!("Joe", s);
            let mut s = String::from("jill");
            make_ascii_titlecase(&mut s);
            assert_eq!("Jill", s);
            let mut s = String::from("von Hagen");
            make_ascii_titlecase(&mut s);
            assert_eq!("Von Hagen", s);
            //assert_eq!("SS", make_ascii_titlecase("ß"));
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
            let mut s = "foobar".to_string();
            let r = s.remove(0).to_uppercase().to_string() + &s;
            println!("{r}");

            let r = format!("{}{s}", s.remove(0).to_uppercase());
            println!("{r}");
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
mod answer4 {
    trait TitleCase {
        fn title(&self) -> String;
    }

    impl TitleCase for &str {
        fn title(&self) -> String {
            if !self.is_ascii() || self.is_empty() {
                return String::from(*self);
            }
            let (head, tail) = self.split_at(1);
            head.to_uppercase() + tail
        }
    }
    pub fn test() {
        println!("{}", "bruno".title());
        println!("{}", "b".title());
        println!("{}", "🦀".title());
        println!("{}", "ß".title());
        println!("{}", "".title());
        println!("{}", "བོད་སྐད་ལ".title());
    }
}
pub fn test() {
    _enter!();
    answer1::test();
    answer2::test();
    answer3::test();
    answer4::test();
    _leave!();
}
