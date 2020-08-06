// Modules for macros except for arguments

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_fmt_for_type{
    ($type_name:ident, $($variant:pat => $description:expr), *) =>{
        impl Display for $type_name{
            fn fmt(&self, f: &mut Formatter) -> fmt::Result{
                match *self{
                    $($variant => write!(f, $description),
                        )*
                }
            }
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_fmt_test{
    ($test_name:ident, $($variant:expr => $description:expr), *) => {
        #[test]
        fn $test_name(){
            $(assert_eq!(format!("{}", $variant).as_str(), $description);
                )*
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_fromstr_for_type{
    ($type_name:ident, $($variant:expr => $description:expr), *) => {
        impl FromStr for $type_name{
            type Err = Error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s{
                    $($description => Ok($variant),
                        )*
                    _ => Err(Error::make_error_syntax(crate::error::ErrorCode::InvalidArgumentInput)),
                }
            }
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_fromstr_test{
    ($test_name:ident, $type_name:ident, $($variant:expr => $description:expr), *) => {
        #[test]
        fn $test_name(){
            $(assert_eq!($type_name::from_str($description), Ok($variant));
              assert_eq!($description.parse::<$type_name>(), Ok($variant));
                )*
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! define_structure{
    ($name:ident $(, $type_name:ident, $type_type:ty)* ;$($var:ident, $t:ty,)*) =>{
        #[derive(Clone, Debug, PartialEq, PartialOrd)]
        pub struct $name{
            $(pub $type_name : $type_type,
                )*
            $(pub $var : $t,
                )*
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! impl_structure{
    ($name:ident $(, $type_name:ident, $type_default:expr)* ;$($var:ident, $t:ty,) *) =>{
        #[allow(dead_code)]
        impl $name{
            pub fn new($($var : $t),*) -> Self{
                Self{
                    $($type_name : $type_default,
                        )*
                    $($var : $var,
                        )*
                }
            }

            pub fn clear(&mut self){
                $(
                    self.$type_name = $type_default;
                    )*
                $(
                    self.$var = Default::default();
                    )*
            }
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! construct_structure {
    ($name:ident $(, $type_name:ident, $type_type:ty, $type_default:expr)* ;$($var:ident, $t:ty), *) => {
        define_structure!($name $(, $type_name, $type_type)* ;$($var, $t,) *);

        impl_structure!($name $(, $type_name, $type_default)* ;$($var, $t,) *);
    };
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! derive_hash{
    ($name:ident $(, $var:ident) *) => {
        impl Hash for $name{
            fn hash<H: Hasher>(&self, state: &mut H){
                $((self.$var as usize).hash(state);
                    )*
            }
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! export_form{
    ($name:ident $(,$var:ident) *) => {
        fn $name(width: usize) -> String{
            let mut string = String::new();
            $(string.push_str(format!("{}", format_args!("{0:<1$}", stringify!($var), width)).as_str());
                )*
            return string;
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! pub_export_form{
    ($name:ident $(,$var:ident) *) => {
        #[allow(dead_code)]
        pub fn $name(width: usize) -> String{
            let mut string = String::new();
            $(string.push_str(format!("{}", format_args!("{0:<1$}", stringify!($var), width)).as_str());
                )*
            return string;
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! export_data{
    ($name:ident $(,$var:ident)*; $($float:ident), *) => {
        #[allow(dead_code)]
        fn $name(&self, prec: usize) -> String{
            let mut string = String::new();
            $(string.push_str(format!("{}", format_args!("{0:<1$}\t", self.$var, prec)).as_str());
                )*
            $(string.push_str(format!("{}", format_args!("{0:.1$e}\t", self.$float , prec)).as_str());
                )*
            return string;
        }
    }
}

#[macro_export]
#[allow(unused_macros)]
macro_rules! pub_export_data{
    ($name:ident $(,$var:ident)*; $($float:ident), *) => {
        #[allow(dead_code)]
        pub fn $name(&self, prec: usize) -> String{
            let mut string = String::new();
            $(string.push_str(format!("{}", format_args!("{0:<1$}\t", self.$var, prec)).as_str());
                )*
            $(string.push_str(format!("{}", format_args!("{0:.1$e}\t", self.$float , prec)).as_str());
                )*
            return string;
        }
    }
}


// #[macro_export]
// #[allow(unused_macros)]
// macro_rules! recursive_export_float{
//     ($string:ident, $self:ident, $prec:ident,) => {};

//     ($string:ident, $self:ident, $prec:ident, $float:ident = f64, $($tt:tt),*) => {
//         $string.push_str(format!("{}", format_args!("{0:.1$e}\t", $self.$float , $prec)).as_str());
//         recursive_export_float!($string, $self, $prec, $($tt:tt),*);
//     };

//     ($string:ident, $self:ident, $prec:ident, $var1:ident, $($tt:tt),*) => {
//         $string.push_str(format!("{}", format_args!("{0:.1$}\t", $self.$var1 , $prec)).as_str());
//         recursive_export_float!($string, $self, $prec, $($tt:tt),*);
//     }
// }

// #[macro_export]
// #[allow(unused_macros)]
// macro_rules! pub_export_data2{
//     ($name:ident, $($tt:tt),*) => {
//         #[allow(dead_code)]
//         pub fn $name(&self, prec: usize) -> String{
//             let mut string = String::new();
//             recursive_export_float!(string, self, prec, $($tt:tt),*);
//             return string;
//         }
//     }
// }



#[cfg(test)]
mod tests{
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_export_data(){
        struct Test{
            num1 : usize,
            num2 : usize,
            float1 : f64,
            name : String,
        }
        impl Test{
            export_data!(test_export, num1, num2, name; float1);
        }

        let test = Test{
            num1 : 10,
            num2 : 5,
            float1: 0f64,
            name : String::from("Hello"),
        };

        assert_eq!(test.test_export(5), "10   \t5    \tHello\t0.00000e0\t".to_string());
    }

    // #[test]
    // fn test_export_data2(){
    //     struct Test{
    //         num1 : usize,
    //         num2 : usize,
    //         float1 : f64,
    //         name : String,
    //     }
    //     impl Test{
    //         pub_export_data2!(test_export, num1, num2, name, float1 = f64);
    //     }

    //     let test = Test{
    //         num1 : 10,
    //         num2 : 5,
    //         float1: 0f64,
    //         name : String::from("Hello"),
    //     };

    //     assert_eq!(test.test_export(5), "10   \t5    \tHello\t0.00000e0\t".to_string());
    // }


}
