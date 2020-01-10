use crate::tree::Tree;

/// A map implemented with prefix tree.
#[derive(Debug, Clone, Default)]
pub struct PrefixTreeMap<K, V> {
    root: Tree<K, V>,
    length: usize,
}

impl<K: Eq + Clone, V> PrefixTreeMap<K, V> {
    /// Creates an empty `PrefixTreeMap`.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixTreeMap;
    ///
    /// let mut map: PrefixTreeMap<u8, i32> = PrefixTreeMap::new();
    /// ```
    pub fn new() -> PrefixTreeMap<K, V> {
        PrefixTreeMap {
            root: Tree::empty(),
            length: 0,
        }
    }

    /// Returns `true` if the map contains a value for the specified key.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixTreeMap;
    ///
    /// let mut map: PrefixTreeMap<u8, i32> = PrefixTreeMap::new();
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
    /// use prefix_tree::PrefixTreeMap;
    ///
    /// let mut map: PrefixTreeMap<u8, i32> = PrefixTreeMap::new();
    /// map.insert("foo", 1);
    /// map.clear();
    /// assert!(map.is_empty());
    /// ```
    pub fn clear(&mut self) {
        *self = PrefixTreeMap::new();
    }

    /// Returns a reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixTreeMap;
    ///
    /// let mut map: PrefixTreeMap<u8, i32> = PrefixTreeMap::new();
    /// map.insert("foo", 1);
    /// assert_eq!(map.get("foo"), Some(&1));
    /// assert_eq!(map.get("bar"), None);
    /// ```
    pub fn get<Q>(&self, key: Q) -> Option<&V>
    where
        Q: AsRef<[K]>,
    {
        self.root.find(key.as_ref()).and_then(|x| x.value.as_ref())
    }

    /// Returns a mutable reference to the value corresponding to the key.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixTreeMap;
    ///
    /// let mut map: PrefixTreeMap<u8, i32> = PrefixTreeMap::new();
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
        self.root
            .find_mut(key.as_ref())
            .and_then(|x| x.value.as_mut())
    }

    /// Inserts a key-value pair into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixTreeMap;
    ///
    /// let mut map: PrefixTreeMap<u8, i32> = PrefixTreeMap::new();
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
    /// use prefix_tree::PrefixTreeMap;
    ///
    /// let mut map: PrefixTreeMap<u8, i32> = PrefixTreeMap::new();
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
    /// use prefix_tree::PrefixTreeMap;
    ///
    /// let mut map: PrefixTreeMap<u8, i32> = PrefixTreeMap::new();
    /// assert_eq!(map.len(), 0);
    /// map.insert("foo", 1);
    /// assert_eq!(map.len(), 1);
    /// ```
    pub fn len(&self) -> usize {
        self.length
    }
}
