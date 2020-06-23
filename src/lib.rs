/// This type exists for one purpose and one purpose only. To help you ensure you're mutating
/// the correct thing, and not mutating a copy. Put `Copy` types in here to rob them of their `Copy` implementation.
///
/// This wrapper tries to be transparent in every way, of course it can't implement every trait in existence,
/// but we'll try to get the important ones. Crate feature "serde" will include `Serialize` and `Deserialize`.
///
/// Major omissions from the trait implementations include common math traits, because consuming a `NotCopy`
/// usually isn't helpful, and those traits consume. However, traits such as `AddAssign` are implemented because
/// they're just a mutation.
#[derive(Default, Clone, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct NotCopy<T>(pub T);

impl<T> From<T> for NotCopy<T>
{
    fn from(t: T) -> Self {
        NotCopy(t.into())
    }
}

impl<T, Rhs> std::ops::AddAssign<Rhs> for NotCopy<T>
where
    T: std::ops::AddAssign<Rhs>,
{
    fn add_assign(&mut self, rhs: Rhs) {
        self.0 += rhs
    }
}

impl<T, Rhs> std::ops::SubAssign<Rhs> for NotCopy<T>
where
    T: std::ops::SubAssign<Rhs>,
{
    fn sub_assign(&mut self, rhs: Rhs) {
        self.0 -= rhs
    }
}

impl<T, Rhs> std::ops::MulAssign<Rhs> for NotCopy<T>
where
    T: std::ops::MulAssign<Rhs>,
{
    fn mul_assign(&mut self, rhs: Rhs) {
        self.0 *= rhs
    }
}

impl<T, Rhs> std::ops::DivAssign<Rhs> for NotCopy<T>
where
    T: std::ops::DivAssign<Rhs>,
{
    fn div_assign(&mut self, rhs: Rhs) {
        self.0 /= rhs
    }
}

impl<T, Rhs> std::ops::RemAssign<Rhs> for NotCopy<T>
where
    T: std::ops::RemAssign<Rhs>,
{
    fn rem_assign(&mut self, rhs: Rhs) {
        self.0 %= rhs
    }
}

impl<T, Rhs> std::ops::ShlAssign<Rhs> for NotCopy<T>
where
    T: std::ops::ShlAssign<Rhs>,
{
    fn shl_assign(&mut self, rhs: Rhs) {
        self.0 <<= rhs
    }
}

impl<T, Rhs> std::ops::ShrAssign<Rhs> for NotCopy<T>
where
    T: std::ops::ShrAssign<Rhs>,
{
    fn shr_assign(&mut self, rhs: Rhs) {
        self.0 >>= rhs
    }
}

impl<T, Rhs> std::ops::BitAndAssign<Rhs> for NotCopy<T>
where
    T: std::ops::BitAndAssign<Rhs>,
{
    fn bitand_assign(&mut self, rhs: Rhs) {
        self.0 &= rhs
    }
}

impl<T, Rhs> std::ops::BitOrAssign<Rhs> for NotCopy<T>
where
    T: std::ops::BitOrAssign<Rhs>,
{
    fn bitor_assign(&mut self, rhs: Rhs) {
        self.0 |= rhs
    }
}

impl<T, Rhs> std::ops::BitXorAssign<Rhs> for NotCopy<T>
where
    T: std::ops::BitXorAssign<Rhs>,
{
    fn bitxor_assign(&mut self, rhs: Rhs) {
        self.0 ^= rhs
    }
}

impl<T, Idx> std::ops::Index<Idx> for NotCopy<T>
where
    T: std::ops::Index<Idx>,
{
    type Output = T::Output;

    fn index(&self, index: Idx) -> &Self::Output {
        &self.0[index]
    }
}

impl<T, Idx> std::ops::IndexMut<Idx> for NotCopy<T>
where
    T: std::ops::IndexMut<Idx>,
{
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<T> std::fmt::Display for NotCopy<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<T> std::fmt::Debug for NotCopy<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
impl<'de, T> serde::Deserialize<'de> for NotCopy<T>
where
    T: serde::Deserialize<'de>,
{
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(NotCopy)
    }
}

#[cfg(feature = "serde")]
impl<T> serde::Serialize for NotCopy<T>
where
    T: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        T::serialize(&self.0, serializer)
    }
}
