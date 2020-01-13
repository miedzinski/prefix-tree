use map::PrefixMap;
use std::iter::FromIterator;

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
