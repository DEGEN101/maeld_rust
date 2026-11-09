#[macro_export]
macro_rules! boxed {
    ($e:expr) => {
        Box::new($e)
    };
}

#[macro_export]
macro_rules! fml_vec {
    ( $( $element:expr),* $(,)? ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(boxed!($element));
            )*
            v
        }
    };
}

pub(crate) use boxed;
pub(crate) use fml_vec;
