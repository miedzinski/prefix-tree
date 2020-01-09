use map::PrefixTreeMap;

#[derive(Debug, Default)]
pub struct PrefixTreeSet<T> {
    map: PrefixTreeMap<T, ()>,
}

impl<T: Eq + Clone> PrefixTreeSet<T> {
    pub fn new() -> PrefixTreeSet<T> {
        PrefixTreeSet {
            map: PrefixTreeMap::new(),
        }
    }

    pub fn contains<Q>(&self, key: Q) -> bool
    where
        Q: AsRef<[T]>,
    {
        self.map.get(key).is_some()
    }

    pub fn insert<Q>(&mut self, key: Q)
    where
        Q: AsRef<[T]>,
    {
        self.map.insert(key, ())
    }
}
