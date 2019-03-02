use std::borrow::Cow;

use crate::{Cons, Number, Value};

macro_rules! impl_from_number {
    (
        $($ty:ty),*
    ) => {
        $(
            impl From<$ty> for Value {
                #[inline]
                fn from(n: $ty) -> Self {
                    Value::Number(Number::from(n))
                }
            }
        )*
    };
}

impl_from_number!(u8, u16, u32, u64, i8, i16, i32, i64, f32, f64);

impl From<&str> for Value {
    #[inline]
    fn from(s: &str) -> Self {
        Value::String(s.into())
    }
}

impl<'a> From<Cow<'a, str>> for Value {
    #[inline]
    fn from(s: Cow<'a, str>) -> Self {
        Value::from(s.to_string())
    }
}

impl From<String> for Value {
    #[inline]
    fn from(s: String) -> Self {
        Value::String(s)
    }
}

impl From<bool> for Value {
    #[inline]
    fn from(v: bool) -> Self {
        Value::Bool(v)
    }
}

impl From<Number> for Value {
    fn from(n: Number) -> Self {
        Value::Number(n)
    }
}

impl<T, U> From<(T, U)> for Value
where
    T: Into<Value>,
    U: Into<Value>,
{
    fn from((car, cdr): (T, U)) -> Self {
        Value::Cons(Cons::new(car, cdr))
    }
}

impl From<Cons> for Value {
    fn from(pair: Cons) -> Self {
        Value::Cons(pair)
    }
}
