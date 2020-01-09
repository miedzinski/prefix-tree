use crate::tree::Tree;

#[derive(Debug, Clone, Default)]
pub struct PrefixTreeMap<K, V> {
    root: Option<Tree<K, V>>,
}

impl<K: Eq + Clone, V> PrefixTreeMap<K, V> {
    pub fn new() -> PrefixTreeMap<K, V> {
        PrefixTreeMap { root: None }
    }
    pub fn contains_key<Q>(&self, key: Q) -> bool
    where
        Q: AsRef<[K]>,
    {
        self.get(key).is_some()
    }

    pub fn clear(&mut self) {
        self.root = None
    }

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
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}
