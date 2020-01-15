use crate::tree::Tree;
use std::iter::{FromIterator, FusedIterator};

/// A map implemented with prefix tree.
#[derive(Debug, Clone, Default)]
pub struct PrefixMap<K, V> {
    root: Tree<K, V>,
    length: usize,
}

impl<K: Eq + Clone, V> PrefixMap<K, V> {
    /// Creates an empty `PrefixMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<u8, i32> = PrefixMap::new();
    /// ```
    pub fn new() -> PrefixMap<K, V> {
        PrefixMap {
            root: Tree::empty(),
            length: 0,
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<u8, i32> = PrefixMap::new();
    /// map.insert("foo", 1);
    /// assert_eq!(map.contains_key("foo"), true);
    /// assert_eq!(map.contains_key("bar"), false);
    /// ```
    pub fn contains_key<Q>(&self, key: Q) -> bool
    where
        Q: AsRef<[K]>,
    {
        self.get(key).is_some()
    }

    /// Clears the map, removing all key-value pairs.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<u8, i32> = PrefixMap::new();
    /// map.insert("foo", 1);
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = PrefixMap::new();
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<u8, i32> = PrefixMap::new();
    /// map.insert("foo", 1);
    /// assert_eq!(map.get("foo"), Some(&1));
    /// assert_eq!(map.get("bar"), None);
    /// ```
    pub fn get<Q>(&self, key: Q) -> Option<&V>
    where
        Q: AsRef<[K]>,
    {
        self.root.find(key.as_ref()).and_then(|x| x.value())
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<u8, i32> = PrefixMap::new();
    /// map.insert("foo", 1);
    /// if let Some(x) = map.get_mut("foo") {
    ///     *x = 2;
    /// }
    /// assert_eq!(map.get("foo"), Some(&2));
    /// ```
    pub fn get_mut<Q>(&mut self, key: Q) -> Option<&mut V>
    where
        Q: AsRef<[K]>,
    {
        self.root.find_mut(key.as_ref()).and_then(|x| x.value_mut())
    }

    /// Inserts a key-value pair into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<u8, i32> = PrefixMap::new();
    /// assert_eq!(map.insert("a", 42), None);
    /// assert_eq!(map.is_empty(), false);
    /// assert_eq!(map.insert("a", 5), Some(42));
    /// assert_eq!(map.get("a"), Some(&5));
    /// ```
    pub fn insert<Q>(&mut self, key: Q, value: V) -> Option<V>
    where
        Q: AsRef<[K]>,
    {
        let old = self.root.insert(key.as_ref(), value);
        if old.is_none() {
            self.length += 1;
        }
        old
    }

    /// Returns `true` if the map contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<u8, i32> = PrefixMap::new();
    /// assert_eq!(map.is_empty(), true);
    /// map.insert("foo", 1);
    /// assert_eq!(map.is_empty(), false);
    /// ```
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the number of elements in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<u8, i32> = PrefixMap::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert("foo", 1);
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }

    /// Gets an iterator over the entries of the map, in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<u8, i32> = PrefixMap::new();
    /// map.insert("1", 9);
    /// map.insert("2", 8);
    /// map.insert("3", 7);
    ///
    /// for (key, value) in map.iter() {
    ///     println!("{:?}: {:?}", key, value);
    /// }
    /// ```
    pub fn iter(&self) -> Iter<K, V> {
        Iter {
            root: &self.root,
            stack: vec![IterStackItem {
                iter: self.root.children().iter(),
                key_fragment: &self.root.key(),
            }],
            length: self.length,
        }
    }

    /// Gets an iterator over the keys of the map, in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<i32, i32> = PrefixMap::new();
    /// map.insert([1], 2);
    /// map.insert([2], 3);
    ///
    /// assert_eq!(map.keys().collect::<Vec<_>>(), vec![vec![1], vec![2]]);
    /// ```
    pub fn keys(&self) -> Keys<K, V> {
        Keys { inner: self.iter() }
    }

    /// Gets an iterator over the values of the map, in arbitrary order.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixMap;
    ///
    /// let mut map: PrefixMap<i32, i32> = PrefixMap::new();
    /// map.insert([1], 2);
    /// map.insert([2], 3);
    ///
    /// assert_eq!(map.values().cloned().collect::<Vec<_>>(), vec![2, 3]);
    /// ```
    pub fn values(&self) -> Values<K, V> {
        Values { inner: self.iter() }
    }
}

impl<'a, K: 'a + Eq + Clone, V: 'a + Clone> FromIterator<(&'a [K], V)> for PrefixMap<K, V> {
    fn from_iter<I>(iter: I) -> PrefixMap<K, V>
    where
        I: IntoIterator<Item = (&'a [K], V)>,
    {
        let mut map = PrefixMap::new();
        iter.into_iter().for_each(|(k, v)| {
            map.insert(k, v);
        });
        map
    }
}

impl<'a, K: 'a + Eq + Clone, V: 'a + Clone> IntoIterator for &'a PrefixMap<K, V> {
    type Item = (Vec<K>, &'a V);

    type IntoIter = Iter<'a, K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

struct IterStackItem<'a, K: 'a, V: 'a> {
    iter: std::slice::Iter<'a, Tree<K, V>>,
    key_fragment: &'a [K],
}

pub struct Iter<'a, K: 'a, V: 'a> {
    root: &'a Tree<K, V>,
    stack: Vec<IterStackItem<'a, K, V>>,
    length: usize,
}

impl<'a, K: 'a + Eq + Clone, V: 'a + Clone> Iterator for Iter<'a, K, V> {
    type Item = (Vec<K>, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.length == 1 && self.root.value().is_some() {
            self.length = 0;
            return self.root.value().map(|x| (vec![], x));
        }
        while let Some(IterStackItem { iter, .. }) = self.stack.last_mut() {
            if let Some(tree) = iter.next() {
                self.stack.push(IterStackItem {
                    iter: tree.children().iter(),
                    key_fragment: tree.key(),
                });
                if tree.value().is_some() {
                    self.length -= 1;
                    return Some((
                        self.stack
                            .iter()
                            .map(|x| x.key_fragment)
                            .flatten()
                            .cloned()
                            .collect(),
                        tree.value().unwrap(),
                    ));
                }
            } else {
                self.stack.pop();
            }
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.length, Some(self.length))
    }
}

impl<K: Eq + Clone, V: Clone> ExactSizeIterator for Iter<'_, K, V> {
    fn len(&self) -> usize {
        self.length
    }
}

impl<K: Eq + Clone, V: Clone> FusedIterator for Iter<'_, K, V> {}

pub struct Keys<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: 'a + Eq + Clone, V: 'a + Clone> Iterator for Keys<'a, K, V> {
    type Item = Vec<K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(k, _)| k)
    }
}

pub struct Values<'a, K: 'a, V: 'a> {
    inner: Iter<'a, K, V>,
}

impl<'a, K: 'a + Eq + Clone, V: 'a + Clone> Iterator for Values<'a, K, V> {
    type Item = &'a V;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(_, v)| v)
    }
}
