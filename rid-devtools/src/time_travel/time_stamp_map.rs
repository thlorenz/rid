use std::{
    collections::BTreeMap,
    ops::{Deref, DerefMut},
};

// TODO(thlorenz): max items stored aka RingBuffer
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

    pub fn remove_items_ge(&mut self, ts: u128) {
        self.0.retain(|&k, _| k <= ts);
    }

    pub fn remove_items_gt(&mut self, ts: u128) {
        self.0.retain(|&k, _| k < ts);
    }

    pub fn item_ge(&self, time_stamp: u128) -> Option<&T> {
        for ts in self.keys() {
            if ts >= &time_stamp {
                return self.get(ts);
            }
        }
        None
    }

    pub fn item_gt(&self, time_stamp: u128) -> Option<&T> {
        for ts in self.keys() {
            if ts > &time_stamp {
                return self.get(ts);
            }
        }
        None
    }

    pub fn item_le(&self, time_stamp: u128) -> Option<&T> {
        for ts in self.keys().rev() {
            if ts <= &time_stamp {
                return self.get(ts);
            }
        }
        None
    }

    pub fn item_lt(&self, time_stamp: u128) -> Option<&T> {
        for ts in self.keys().rev() {
            if ts < &time_stamp {
                return self.get(ts);
            }
        }
        None
    }

    pub fn items_up_to(&self, time_stamp: u128) -> Vec<&T> {
        let mut items: Vec<&T> = vec![];
        for (ts, item) in self.iter() {
            if ts > &time_stamp {
                break;
            }
            items.push(item)
        }
        items
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
    fn item_ge() {
        let tsm = setup();

        assert_eq!(tsm.item_ge(1), Some(&First));
        assert_eq!(tsm.item_ge(2), Some(&Second));
        assert_eq!(tsm.item_ge(5), Some(&Second));
        assert_eq!(tsm.item_ge(6), Some(&Third));
        assert_eq!(tsm.item_ge(80), Some(&Fifth));
        assert_eq!(tsm.item_ge(100), None);
    }

    #[test]
    fn item_gt() {
        let tsm = setup();

        assert_eq!(tsm.item_gt(1), Some(&Second));
        assert_eq!(tsm.item_gt(2), Some(&Second));
        assert_eq!(tsm.item_gt(5), Some(&Third));
        assert_eq!(tsm.item_gt(80), None);
        assert_eq!(tsm.item_gt(100), None);
    }

    #[test]
    fn item_le() {
        let tsm = setup();

        assert_eq!(tsm.item_le(1), Some(&First));
        assert_eq!(tsm.item_le(2), Some(&First));
        assert_eq!(tsm.item_le(5), Some(&Second));
        assert_eq!(tsm.item_le(6), Some(&Second));
        assert_eq!(tsm.item_le(80), Some(&Fifth));
        assert_eq!(tsm.item_le(81), Some(&Fifth));
        assert_eq!(tsm.item_le(100), Some(&Fifth));
    }
    #[test]
    fn item_lt() {
        let tsm = setup();

        assert_eq!(tsm.item_lt(1), None);
        assert_eq!(tsm.item_lt(2), Some(&First));
        assert_eq!(tsm.item_lt(5), Some(&First));
        assert_eq!(tsm.item_lt(6), Some(&Second));
        assert_eq!(tsm.item_lt(80), Some(&Fourth));
        assert_eq!(tsm.item_lt(81), Some(&Fifth));
        assert_eq!(tsm.item_lt(100), Some(&Fifth));
    }

    #[test]
    fn items_up_to() {
        let tsm = setup();

        assert_eq!(tsm.items_up_to(2), vec![&First]);
        assert_eq!(tsm.items_up_to(5), vec![&First, &Second]);
        assert_eq!(tsm.items_up_to(6), vec![&First, &Second]);
        assert_eq!(tsm.items_up_to(70), vec![&First, &Second, &Third, &Fourth]);
        assert_eq!(
            tsm.items_up_to(80),
            vec![&First, &Second, &Third, &Fourth, &Fifth]
        );
        assert_eq!(
            tsm.items_up_to(90),
            vec![&First, &Second, &Third, &Fourth, &Fifth]
        );
    }
}
