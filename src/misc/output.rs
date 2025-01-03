use std::fmt::{self, Display};


#[derive(fmt::Debug, Clone)]
pub enum OutputValue {
    I8(i8),
    U8(u8),
    I16(i16),
    U16(u16),
    I32(i32),
    U32(u32),
    I64(i64),
    U64(u64),
    ISize(isize),
    USize(usize),
    I128(i128),
    U128(u128),
    F32(f32),
    F64(f64),
    String(String)
}


macro_rules! impl_from {
    ($obj:ident, $(($t:ty, $name:ident)),+ $(,)?) => {
        $(
            impl From<$t> for $obj {
                fn from(n: $t) -> Self {
                    $obj::$name(n)
                }
            }
        )+
    };
}

impl_from!(OutputValue,
    (i8, I8), (u8, U8),
    (i16, I16), (u16, U16),
    (i32, I32), (u32, U32),
    (i64, I64), (u64, U64),
    (isize, ISize), (usize, USize),
    (i128, I128), (u128, U128),
    (f32, F32),
    (f64, F64),
    (String, String)
);
impl From<&str> for OutputValue {
    fn from(n: &str) -> Self {
        Self::String(n.to_string())
    }
}


impl Display for OutputValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::I8(n) => n.fmt(f),
            Self::U8(n) => n.fmt(f),
            Self::I16(n) => n.fmt(f),
            Self::U16(n) => n.fmt(f),
            Self::I32(n) => n.fmt(f),
            Self::U32(n) => n.fmt(f),
            Self::I64(n) => n.fmt(f),
            Self::U64(n) => n.fmt(f),
            Self::ISize(n) => n.fmt(f),
            Self::USize(n) => n.fmt(f),
            Self::I128(n) => n.fmt(f),
            Self::U128(n) => n.fmt(f),
            Self::F32(n) => n.fmt(f),
            Self::F64(n) => n.fmt(f),
            Self::String(n) => n.fmt(f)
        }
    }
}
