#[cfg(test)]
mod tests {
    use super::AnyBox;

    macro_rules! typed_test {
        ($name:ident, $ty:ty, $value:expr) => {
            #[test]
            fn $name() {
                let data: $ty = $value;
                let expected: $ty = $value;

                let anybox = AnyBox::new(data);

                assert_eq!(anybox.get::<$ty>(), &expected);
                assert_eq!(anybox.try_get::<()>(), None);
            }
        };
    }

    typed_test!(byte, u8, 1);
    typed_test!(byte2, u8, 123);

    typed_test!(usize1, usize, 1);
    typed_test!(usize2, usize, 2);
    typed_test!(usize3, usize, 26387890);
    typed_test!(usize4, usize, 123789123);
    typed_test!(usize5, usize, 3279173921873921);

    typed_test!(isize1, isize, 1);
    typed_test!(isize2, isize, 2);
    typed_test!(isize3, isize, 26387890);
    typed_test!(isize4, isize, 123789123);
    typed_test!(isize5, isize, 3279173921873921);
    typed_test!(neg_isize1, isize, -1);
    typed_test!(neg_isize2, isize, -2);
    typed_test!(neg_isize3, isize, -26387890);
    typed_test!(neg_isize4, isize, -123789123);
    typed_test!(neg_isize5, isize, -3279173921873921);
}

use std::any;

/// Container for values of any type.
/// When retrieving values, correct type has to be specified.
pub struct AnyBox {
    ty: any::TypeId,
    // not really pointing to values of type u8.
    // rather it's used as a pointer to any sort of data.
    value: Box<u8>,
}

impl AnyBox {
    pub fn new<T: 'static>(value: T) -> Self {
        let ty = any::TypeId::of::<T>();

        // place value on the heap
        let value = Box::new(value);
        let value = unsafe { std::mem::transmute::<Box<T>, Box<u8>>(value) };
        Self { ty, value }
    }

    /// get a reference to the held value or None
    pub fn try_get<'a, T: 'static>(&'a self) -> Option<&'a T> {
        let ty = any::TypeId::of::<T>();

        if ty != self.ty {
            return None;
        }

        let value = unsafe { std::mem::transmute::<&u8, &T>(self.value.as_ref()) };

        Some(value)
    }

    /// get a reference to the held value or panic
    pub fn get<'a, T: 'static>(&'a self) -> &T {
        match self.try_get::<T>() {
            None => {
                let full_name = any::type_name::<T>();
                panic!("Attempted to take value as forgein type {}", full_name);
            }
            Some(t) => t,
        }
    }
}
