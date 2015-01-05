#![feature(macro_rules)]
#![macro_escape]

#[macro_export]
macro_rules! data {
    ($structtype:ident {$($var:ident: $vartype:ty = $val:expr)+}) => {
        pub struct $structtype {
            $(
                pub $var: $vartype,
            )+
        }

        impl $structtype {
            pub fn new() -> $structtype {
                $(
                    let $var = $val;
                )+
                $structtype {
                    $(
                        $var: $var,
                    )+
                }
            }
        }
    }
}
