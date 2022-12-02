// @filename   : stackoverflow_0038_19650265.rs
// @date       : Wed Nov 16 12:31:09 PM HKT 2022
// @author     : Church.ZHONG
// @see        : https://stackoverflow.com/questions/19650265
// @title      : Is there a faster/shorter way to initialize variables in a Rust struct?

#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(dead_code)]
mod answer1 {
    mod code1 {
        #[derive(Debug)]
        struct cParams {
            iInsertMax: i64,
            iUpdateMax: i64,
            iDeleteMax: i64,
            iInstanceMax: i64,
            tFirstInstance: bool,
            tCreateTables: bool,
            tContinue: bool,
        }

        impl Default for cParams {
            fn default() -> cParams {
                cParams {
                    iInsertMax: -1,
                    iUpdateMax: -1,
                    iDeleteMax: -1,
                    iInstanceMax: -1,
                    tFirstInstance: false,
                    tCreateTables: false,
                    tContinue: false,
                }
            }
        }
        pub fn test() {
            // add your code here
            let p = cParams {
                iInsertMax: 10,
                ..Default::default()
            };
            println!("{:?}", p);
        }
    }
    mod code2 {
        #[derive(Debug, Default)]
        struct cParams {
            iInsertMax: Option<u64>,
            iUpdateMax: Option<u64>,
            iDeleteMax: Option<u64>,
            iInstanceMax: Option<u64>,
            tFirstInstance: bool,
            tCreateTables: bool,
            tContinue: bool,
        }
        pub fn test() {
            // add your code here
            let p = cParams {
                iInsertMax: Some(10),
                ..Default::default()
            };
            println!("{:?}", p);
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
