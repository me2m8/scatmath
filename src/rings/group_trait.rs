use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

pub trait AdditiveInverse: Neg<Output = Self> {}

pub trait AdditiveIdentity: Sized {
    const ZERO: Self;
    fn zero() -> Self;
}

pub trait EqSupport:
    Eq + PartialEq + for<'a> PartialEq<&'a Self> + for<'a> PartialEq<&'a mut Self>
{
}

pub trait AddSupport:
    Sized
    + Add<Output = Self>
    + AddAssign
    + for<'a> Add<&'a Self, Output = Self>
    + for<'a> AddAssign<&'a Self>
    + for<'a> Add<&'a mut Self, Output = Self>
    + for<'a> AddAssign<&'a mut Self>
{
}

pub trait SubSupport:
    Sized
    + Sub<Output = Self>
    + SubAssign
    + for<'a> Sub<&'a Self, Output = Self>
    + for<'a> SubAssign<&'a Self>
    + for<'a> Sub<&'a mut Self, Output = Self>
    + for<'a> SubAssign<&'a mut Self>
{
}

pub trait AdditiveGroup:
    AdditiveInverse + AdditiveIdentity + AddSupport + SubSupport + EqSupport
{
}

pub trait MultiplicativeInverse {
    fn inverse(&self) -> Self;
}

pub trait MaybeMultiplicativeInverse: Sized + MultiplicativeIdentity {
    /// Returns an option containing the inverse of `self` if it exists. Otherwise returns `None`.
    fn inverse(&self) -> Option<Self>;
}

pub trait MultiplicativeIdentity {
    fn one() -> Self;
}

pub trait MulSupport:
    Sized
    + Mul<Output = Self>
    + MulAssign
    + for<'a> Mul<&'a Self, Output = Self>
    + for<'a> MulAssign<&'a Self>
    + for<'a> Mul<&'a mut Self, Output = Self>
    + for<'a> MulAssign<&'a mut Self>
{
}

pub trait DivSupport:
    Sized
    + Div
    + DivAssign
    + for<'a> Div<&'a Self>
    + for<'a> DivAssign<&'a Self>
    + for<'a> Div<&'a mut Self>
    + for<'a> DivAssign<&'a mut Self>
{
}

pub trait MultiplicativeGroup:
    MultiplicativeInverse + MultiplicativeIdentity + MulSupport + DivSupport + EqSupport
{
}
