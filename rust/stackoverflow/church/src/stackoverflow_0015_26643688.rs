// @filename   : stackoverflow_0015_26643688.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/26643688
// @title      : How do I split a string in Rust?

#[allow(dead_code)]
mod answer1 {
    mod code1 {
        pub fn test() {
            // add your code here
            let split = "some string 123 ffd".split("123");
            for each in split {
                println!("{}", each)
            }

            let split = "some string 123 ffd".split("123");
            let vec = split.collect::<Vec<&str>>();
            println!("{:?}", vec);
            // OR
            let split = "some string 123 ffd".split("123");
            let vec: Vec<&str> = split.collect();
            println!("{:?}", vec);
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
        use regex::Regex;
        pub fn test() {
            // add your code here
            let s = "a separator b / c 100 d , 18";

            // By separator:
            let r = s.split("separator");
            println!("{:?}", r);
            let r = s.split('/');
            println!("{:?}", r);
            let r = s.split(char::is_numeric);
            println!("{:?}", r);

            // By whitespace:
            let r = s.split_whitespace();
            println!("{:?}", r);

            // By newlines:
            let r = s.lines();
            println!("{:?}", r);

            // By regex: (using regex crate)
            let binding = Regex::new(r"\s").unwrap();
            println!("{:?}", binding.split("one two three"));

            // The result of each kind is an iterator:

            let text = "foo\r\nbar\n\nbaz\n";
            let mut lines = text.lines();

            assert_eq!(Some("foo"), lines.next());
            assert_eq!(Some("bar"), lines.next());
            assert_eq!(Some(""), lines.next());
            assert_eq!(Some("baz"), lines.next());

            assert_eq!(None, lines.next());
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
        // fn split<'a, P>(&'a self, pat: P) -> Split<'a, P> where P: Pattern<'a>
        pub fn test() {
            // add your code here
            // Split by char:
            let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
            assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

            // Split by string:
            let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
            assert_eq!(v, ["lion", "tiger", "leopard"]);

            // Split by closure:
            let v: Vec<&str> = "abc1def2ghi".split(|c: char| c.is_numeric()).collect();
            assert_eq!(v, ["abc", "def", "ghi"]);
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
