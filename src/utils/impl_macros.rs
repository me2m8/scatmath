
#[macro_export]
macro_rules! impl_add_op {
    (
        type = $ty:ty,
        rhs = $rhs:ty,
        out = $out:ty,
        func = $func:path,
        bounds = [$($bounds:tt)*]
    ) => {
        impl<$($bounds)*> Add<$rhs> for $ty {
            type Output = $out;

            fn add(self, rhs: $rhs) -> Self::Output {
                $func(&self, &rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_add_assign_op {
    (
        type = $ty:ty,
        rhs = $rhs:ty,
        func = $func:path,
        bounds = [$($bounds:tt)*]
    ) => {
        impl<$($bounds)*> AddAssign<$rhs> for $ty {
            fn add_assign(&mut self, rhs: $rhs) {
                $func(self, &rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_sub_op {
    (
        type = $ty:ty,
        rhs = $rhs:ty,
        out = $out:ty,
        func = $func:path,
        bounds = [$($bounds:tt)*]
    ) => {
        impl<$($bounds)*> Sub<$rhs> for $ty {
            type Output = $out;

            fn sub(self, rhs: $rhs) -> Self::Output {
                $func(&self, &rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_sub_assign_op {
    (
        type = $ty:ty,
        rhs = $rhs:ty,
        func = $func:path,
        bounds = [$($bounds:tt)*]
    ) => {
        impl<$($bounds)*> SubAssign<$rhs> for $ty {
            fn sub_assign(&mut self, rhs: $rhs) {
                $func(self, &rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mul_op {
    (
        type = $ty:ty,
        rhs = $rhs:ty,
        out = $out:ty,
        func = $func:path,
        bounds = [$($bounds:tt)*]
    ) => {
        impl<$($bounds)*> Mul<$rhs> for $ty {
            type Output = $out;

            fn mul(self, rhs: $rhs) -> Self::Output {
                $func(&self, &rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_mul_assign_op {
    (
        type = $ty:ty,
        rhs = $rhs:ty,
        func = $func:path,
        bounds = [$($bounds:tt)*]
    ) => {
        impl<$($bounds)*> MulAssign<$rhs> for $ty {
            fn mul_assign(&mut self, rhs: $rhs) {
                $func(self, &rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_div_op {
    (
        type = $ty:ty,
        rhs = $rhs:ty,
        out = $out:ty,
        func = $func:path,
        bounds = [$($bounds:tt)*]
    ) => {
        impl<$($bounds)*> Div<$rhs> for $ty {
            type Output = $out;

            fn div(self, rhs: $rhs) -> Self::Output {
                $func(&self, &rhs)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_eq {
    ($type:ty, $func:path, [$($bounds:tt)*]) => {
        impl<$($bounds)*> PartialEq for $type {
            fn eq(&self, other: &Self) -> bool {
                $func(self, other)
            }
        }
        impl<$($bounds)*> PartialEq<&$type> for $type {
            fn eq(&self, other: &&$type) -> bool {
                $func(self, other)
            }
        }
        impl<$($bounds)*> PartialEq<&mut $type> for $type {
            fn eq(&self, other: &&mut $type) -> bool {
                $func(self, other)
            }
        }
        impl<$($bounds)*> Eq for $type {}
    };
}

#[macro_export]
macro_rules! impl_op {
    (
        $op_impl:ident,
        $type:ty,
        $rhs:ty,
        $func:path,
        [$($bounds:tt)*]
    ) => {
        $op_impl!(type = $type, rhs = $rhs, out = $type, func = $func, bounds = [$($bounds)*]);
        $op_impl!(type = $type, rhs = &$rhs, out = $type, func = $func, bounds = [$($bounds)*]);
        $op_impl!(type = $type, rhs = &mut $rhs, out = $type, func = $func, bounds = [$($bounds)*]);
        $op_impl!(type = &$type, rhs = $rhs, out = $type, func = $func, bounds = [$($bounds)*]);
        $op_impl!(type = &$type, rhs = &$rhs, out = $type, func = $func, bounds = [$($bounds)*]);
        $op_impl!(type = &$type, rhs = &mut $rhs, out = $type, func = $func, bounds = [$($bounds)*]);
        $op_impl!(type = &mut $type, rhs = $rhs, out = $type, func = $func, bounds = [$($bounds)*]);
        $op_impl!(type = &mut $type, rhs = &$rhs, out = $type, func = $func, bounds = [$($bounds)*]);
        $op_impl!(type = &mut $type, rhs = &mut $rhs, out = $type, func = $func, bounds = [$($bounds)*]);
    };
}

#[macro_export]
macro_rules! impl_assign_op {
    ($impl_op:ident, $type:ty, $rhs:ty, $func:path) => {
        $impl_op!(type=$type, rhs=$rhs, func=$func, bounds=[]);
        $impl_op!(type=$type, rhs=&$rhs, func=$func, bounds=[]);
        $impl_op!(type=$type, rhs=&mut $rhs, func=$func, bounds=[]);

        $impl_op!(type=&mut $type, rhs=$rhs, func=$func, bounds=[]);
        $impl_op!(type=&mut $type, rhs=&$rhs, func=$func, bounds=[]);
        $impl_op!(type=&mut $type, rhs=&mut $rhs, func=$func, bounds=[]);
    };
}
