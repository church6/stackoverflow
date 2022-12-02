// use std::time::Instant;
// https://stackoverflow.com/questions/38088067
macro_rules! _function {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            // 'church
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        &name[..name.len() - 3]
    }};
}

macro_rules! _trace {
    ($x:expr) => {
        println!(
            "[{}][{}] {} (in {} [{}:{}:{}])",
            chrono::Local::now(),
            $x,
            _function!(),
            module_path!(),
            file!(),
            line!(),
            column!()
        );
    };
}

// #[macro_export]
macro_rules! _enter {
    () => {
        _trace!("enter")
    };
}
// pub(crate) use _enter;

// #[macro_export]
macro_rules! _leave {
    () => {
        _trace!("leave")
    };
}
// pub(crate) use _leave;
