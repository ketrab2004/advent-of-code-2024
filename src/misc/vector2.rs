#![allow(dead_code)]

pub  trait Directions: Sized {
    const DIRECTIONS: [(Self, Self); 4];
    const DIAGONAL_DIRECTIONS: [(Self, Self); 8];
}

macro_rules! impl_directions {
    ($($t:ty),+ $(,)?) => {
        $(
            impl Directions for $t {
                const DIRECTIONS: [(Self, Self); 4] = [
                    (1 as $t, 0 as $t),
                    (0 as $t, 1 as $t),
                    (-1 as $t, 0 as $t),
                    (0 as $t, -1 as $t)
                ];

                const DIAGONAL_DIRECTIONS: [(Self, Self); 8] = [
                    (1 as $t, 0 as $t), (1 as $t, 1 as $t),
                    (0 as $t, 1 as $t), (-1 as $t, 1 as $t),
                    (-1 as $t, 0 as $t), (-1 as $t, -1 as $t),
                    (0 as $t, -1 as $t), (1 as $t, -1 as $t)
                ];
            }
        )+
    };
}
impl_directions!(i8, i16, i32, i64, isize, f32, f64);

pub const fn directions<T: Directions>() -> [(T, T); 4] {
    T::DIRECTIONS
}
pub const fn diagonal_directions<T: Directions>() -> [(T, T); 8] {
    T::DIAGONAL_DIRECTIONS
}
