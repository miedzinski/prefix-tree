use crate::tree::Tree;

/// A map implemented with prefix tree.
#[derive(Debug, Clone, Default)]
pub struct PrefixTreeMap<K, V> {
    root: Option<Tree<K, V>>,
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
        PrefixTreeMap { root: None }
    }

    /// Returns `true` if the map contains a value for the specifiec key.
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
        self.root = None
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
        self.root
            .as_ref()
            .and_then(|x| x.find(key.as_ref()))
            .map(|x| x.value.as_ref())
            .flatten()
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
            .as_mut()
            .and_then(|x| x.find_mut(key.as_ref()))
            .map(|x| x.value.as_mut())
            .flatten()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use prefix_tree::PrefixTreeMap;
    ///
    /// let mut map: PrefixTreeMap<u8, i32> = PrefixTreeMap::new();
    /// map.insert("a", 42);
    /// assert_eq!(map.is_empty(), false);
    /// map.insert("a", 5);
    /// assert_eq!(map.get("a"), Some(&5));
    /// ```
    pub fn insert<Q>(&mut self, key: Q, value: V)
    where
        Q: AsRef<[K]>,
    {
        if let Some(ref mut root) = self.root {
            root.insert(key.as_ref(), value);
        } else {
            self.root = Some(Tree::new(key.as_ref().to_vec(), value));
        }
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
        self.root.is_none()
    }
}
