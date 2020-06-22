/// This type exists for one purpose and one purpose only. To help you ensure you're mutating
/// the correct thing, and not mutating a copy. Put copy types in here to rob them of their `Copy` status.
#[derive(Default, Clone, Debug)]
pub struct NotCopy<T>(pub T);

impl<T> From<T> for NotCopy<T> {
    fn from(t: T) -> Self {
        NotCopy(t)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into() {
        let _: NotCopy<i32> = 5.into();
    }

    #[test]
    fn from() {
        let _: i32 = NotCopy(5i32).0;
    }
}