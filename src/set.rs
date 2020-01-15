use map::{Iter as MapIter, PrefixMap};
use std::iter::{FromIterator, FusedIterator};

/// A set implemented as a `PrefixMap` where the value is `()`.
#[derive(Debug, Default)]
pub struct PrefixSet<T> {
    map: PrefixMap<T, ()>,
}

impl<T: Eq + Clone> PrefixSet<T> {
    /// Creates an empty `PrefixSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixSet;
    ///
    /// let mut set: PrefixSet<u8> = PrefixSet::new();
    /// ```
    pub fn new() -> PrefixSet<T> {
        PrefixSet {
            map: PrefixMap::new(),
        }
    }

    /// Clears the set, removing all key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixSet;
    ///
    /// let mut set: PrefixSet<u8> = PrefixSet::new();
    /// set.insert("foo");
    /// set.clear();
    /// assert!(set.is_empty());
    /// ```
    pub fn clear(&mut self) {
        self.map.clear()
    }

    /// Returns `true` if the set contains a value.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixSet;
    ///
    /// let mut set: PrefixSet<u8> = PrefixSet::new();
    /// set.insert("1");
    /// assert_eq!(set.contains("1"), true);
    /// assert_eq!(set.contains("2"), false);
    /// ```
    pub fn contains<Q>(&self, key: Q) -> bool
    where
        Q: AsRef<[T]>,
    {
        self.map.get(key).is_some()
    }

    /// Adds a value to the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixSet;
    ///
    /// let mut set: PrefixSet<u8> = PrefixSet::new();
    /// assert_eq!(set.insert("1"), true);
    /// assert_eq!(set.insert("1"), false);
    /// assert_eq!(set.contains("1"), true);
    /// ```
    pub fn insert<Q>(&mut self, key: Q) -> bool
    where
        Q: AsRef<[T]>,
    {
        self.map.insert(key, ()).is_none()
    }

    /// Returns `true` if the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixSet;
    ///
    /// let mut set: PrefixSet<u8> = PrefixSet::new();
    /// assert_eq!(set.is_empty(), true);
    /// set.insert("foo");
    /// assert_eq!(set.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixSet;
    ///
    /// let mut set: PrefixSet<u8> = PrefixSet::new();
    /// assert_eq!(set.len(), 0);
    /// set.insert("foo");
    /// assert_eq!(set.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.map.len()
    }

    /// Gets an iterator that visits the values in `PrefixSet`.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixSet;
    ///
    /// let mut set: PrefixSet<u8> = PrefixSet::new();
    /// set.insert("1");
    /// set.insert("2");
    /// let mut iter = set.iter();
    /// assert_eq!(iter.next(), Some(vec![b'1']));
    /// assert_eq!(iter.next(), Some(vec![b'2']));
    /// ```
    pub fn iter(&self) -> Iter<T> {
        Iter {
            iter: self.map.iter(),
        }
    }
}

impl<'a, T: 'a + Eq + Clone> FromIterator<&'a [T]> for PrefixSet<T> {
    fn from_iter<I>(iter: I) -> PrefixSet<T>
    where
        I: IntoIterator<Item = &'a [T]>,
    {
        let mut set = PrefixSet::new();
        iter.into_iter().for_each(|x| {
            set.insert(x);
        });
        set
    }
}

impl<'a, T: 'a + Eq + Clone> IntoIterator for &'a PrefixSet<T> {
    type Item = Vec<T>;

    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct Iter<'a, T> {
    iter: MapIter<'a, T, ()>,
}

impl<'a, T: 'a + Eq + Clone> Iterator for Iter<'a, T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

impl<'a, T: 'a + Eq + Clone> ExactSizeIterator for Iter<'a, T> {
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<'a, T: 'a + Eq + Clone> FusedIterator for Iter<'a, T> {}
