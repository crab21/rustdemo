#![crate_type = "proc-macro"]


// #[macro_export]
macro_rules! map {
    ($($k:expr=>$v:expr),*) => {
        {
            let mut maps =      ::std::collections::HashMap::new();
            $(
                maps.insert($k,$v);
            )*
            maps
        }
    };
}
