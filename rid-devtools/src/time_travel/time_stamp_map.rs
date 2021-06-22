use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

pub struct TimeStampMap<T>(BTreeMap<u128, T>);
impl<T> Deref for TimeStampMap<T> {
    type Target = BTreeMap<u128, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<T> DerefMut for TimeStampMap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> TimeStampMap<T> {
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn item_right_after(&self, time_stamp: u128) -> Option<&T> {
        for ts in self.keys() {
            if ts > &time_stamp {
                return self.get(ts);
            }
        }
        None
    }

    pub fn item_right_before(&self, time_stamp: u128) -> Option<&T> {
        for ts in self.keys().rev() {
            if ts < &time_stamp {
                return self.get(ts);
            }
        }
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    enum Item {
        First,
        Second,
        Third,
        Fourth,
        Fifth,
    }
    use Item::*;

    fn setup() -> TimeStampMap<Item> {
        let mut tsm = TimeStampMap::<Item>::new();
        tsm.insert(1, First);
        tsm.insert(5, Second);
        tsm.insert(8, Third);
        tsm.insert(9, Fourth);
        tsm.insert(80, Fifth);

        tsm
    }

    #[test]
    fn item_right_after() {
        let tsm = setup();

        assert_eq!(tsm.item_right_after(1), Some(&Second));
        assert_eq!(tsm.item_right_after(2), Some(&Second));
        assert_eq!(tsm.item_right_after(5), Some(&Third));
        assert_eq!(tsm.item_right_after(80), None);
        assert_eq!(tsm.item_right_after(100), None);
    }

    #[test]
    fn item_right_before() {
        let tsm = setup();

        assert_eq!(tsm.item_right_before(1), None);
        assert_eq!(tsm.item_right_before(2), Some(&First));
        assert_eq!(tsm.item_right_before(5), Some(&First));
        assert_eq!(tsm.item_right_before(6), Some(&Second));
        assert_eq!(tsm.item_right_before(80), Some(&Fourth));
        assert_eq!(tsm.item_right_before(81), Some(&Fifth));
        assert_eq!(tsm.item_right_before(100), Some(&Fifth));
    }
}
