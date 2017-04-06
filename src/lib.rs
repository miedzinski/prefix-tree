pub struct Node<T> {
    key: Vec<u8>,
    pub value: Option<T>,
    child: Option<Box<Node<T>>>,
    sibling: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    pub fn new<K: Into<Vec<u8>>>(key: K, value: T) -> Node<T> {
        Node {
            key: key.into(),
            value: Some(value),
            child: None,
            sibling: None,
        }
    }

    fn boxed<K: Into<Vec<u8>>>(key: K, value: T) -> Box<Node<T>> {
        Box::new(Self::new(key, value))
    }

    fn common_prefix<K: AsRef<[u8]>>(&self, other: K) -> usize {
        self.key.iter()
            .zip(other.as_ref().into_iter())
            .take_while(|&(a, b)| a == b)
            .count()
    }

    pub fn find<K: AsRef<[u8]>>(&self, key: K) -> Option<&Node<T>> {
        let key = key.as_ref();
        let prefix = self.common_prefix(key);
        if prefix == 0 {
            self.sibling.as_ref().and_then(|x| x.find(key))
        } else if prefix == self.key.len() {
            if prefix == key.len() {
                Some(self)
            } else {
                self.child.as_ref().and_then(|x| x.find(&key[prefix..]))
            }
        } else {
            None
        }
    }

    pub fn find_mut<K: AsRef<[u8]>>(&mut self, key: K) -> Option<&mut Node<T>> {
        let key = key.as_ref();
        let prefix = self.common_prefix(key);
        if prefix == 0 {
            self.sibling.as_mut().and_then(|x| x.find_mut(key))
        } else if prefix == self.key.len() {
            if prefix == key.len() {
                Some(self)
            } else {
                self.child.as_mut().and_then(|x| x.find_mut(&key[prefix..]))
            }
        } else {
            None
        }
    }

    pub fn insert<K: AsRef<[u8]>>(&mut self, key: K, value: T) {
        let key = key.as_ref();
        let prefix = self.common_prefix(key);
        if prefix == 0 {
            match self.sibling {
                Some(ref mut sibling) => sibling.insert(key, value),
                _ => self.sibling = Some(Self::boxed(key, value)),
            }
        } else if prefix < key.len() {
            if prefix < self.key.len() {
                self.child = Some(Box::new(Node {
                    key: self.key.split_off(prefix),
                    value: self.value.take(),
                    child: self.child.take(),
                    sibling: None,
                }));
                self.key.shrink_to_fit()
            }
            match self.child {
                Some(ref mut child) => child.insert(&key[prefix..], value),
                _ => self.child = Some(Self::boxed(&key[prefix..], value)),
            }
        }
    }
}

pub struct Tree<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> Tree<T> {
    pub fn new() -> Tree<T> {
        Tree {
            root: None,
        }
    }

    pub fn find<K: AsRef<[u8]>>(&self, key: K) -> Option<&Node<T>> {
        self.root.as_ref().and_then(|x| x.find(key))
    }

    pub fn find_mut<K: AsRef<[u8]>>(&mut self, key: K) -> Option<&mut Node<T>> {
        self.root.as_mut().and_then(|x| x.find_mut(key))
    }

    pub fn insert<K: AsRef<[u8]>>(&mut self, key: K, value: T) {
        match self.root {
            Some(ref mut root) => root.insert(key, value),
            _ => self.root = Some(Node::boxed(key.as_ref(), value))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Node, Tree};

    #[test]
    fn test_common_prefix_empty() {
        assert!(Node::new("foo", ()).common_prefix("") == 0);
    }

    #[test]
    fn test_common_prefix_short() {
        assert!(Node::new("foobar", ()).common_prefix("fo0bar") == 2);
    }

    #[test]
    fn test_common_prefix_bytes() {
        let left = "foó";   // [b'f', b'o', b'\xc3', b'\xb3']
        let right = "foò";  // [b'f', b'o', b'\xc3', b'\xb2']
        assert!(Node::new(left, ()).common_prefix(right) == 3);
    }

    #[test]
    fn test_find_empty() {
        let t = Tree::<()> { root: None };
        assert!(t.find("").is_none());
        assert!(t.find("foo").is_none());
    }

    #[test]
    fn test_find_mut_empty() {
        let mut t = Tree::<()> { root: None };
        assert!(t.find_mut("").is_none());
        assert!(t.find_mut("foo").is_none());
    }

    fn sample_tree() -> Tree<i32> {
        Tree {
            root: Some(Box::new(Node {
                key: "fo".to_string().into_bytes(),
                value: Some(0),
                child: Some(Box::new(Node {
                    key: "o".to_string().into_bytes(),
                    value: Some(1),
                    child: None,
                    sibling: Some(Node::boxed("0", 2))
                })),
                sibling: Some(Node::boxed("bar", 3)),
            })),
        }
    }

    #[test]
    fn test_find_simple() {
        assert!(sample_tree().find("fo").unwrap().value == Some(0));
    }

    #[test]
    fn test_find_mut_simple() {
        assert!(sample_tree().find_mut("fo").unwrap().value == Some(0));
    }

    #[test]
    fn test_find_child() {
        assert!(sample_tree().find("foo").unwrap().value == Some(1));
    }

    #[test]
    fn test_find_mut_child() {
        assert!(sample_tree().find_mut("foo").unwrap().value == Some(1));
    }

    #[test]
    fn test_find_sibling() {
        assert!(sample_tree().find("bar").unwrap().value == Some(3));
    }

    #[test]
    fn test_find_mut_sibling() {
        assert!(sample_tree().find_mut("bar").unwrap().value == Some(3));
    }

    #[test]
    fn test_find_missing() {
        assert!(sample_tree().find("quux").is_none());
    }

    #[test]
    fn test_find_mut_missing() {
        assert!(sample_tree().find_mut("quux").is_none());
    }

    #[test]
    fn test_find_shorter() {
        assert!(sample_tree().find("f").is_none());
    }

    #[test]
    fn test_find_mut_shorter() {
        assert!(sample_tree().find_mut("f").is_none());
    }

    #[test]
    fn test_find_longer() {
        assert!(sample_tree().find("fooo").is_none());
    }

    #[test]
    fn test_find_mut_longer() {
        assert!(sample_tree().find_mut("fooo").is_none());
    }

    #[test]
    fn test_insert_empty() {
        let mut t = Tree::new();
        t.insert("foo", ());
        let root = t.root.as_ref().unwrap();
        assert!(root.key == b"foo");
        assert!(root.value == Some(()));
        assert!(root.child.is_none());
        assert!(root.sibling.is_none());
    }

    #[test]
    fn test_insert_append() {
        let mut t = Tree::new();
        t.insert("foo", 0);
        t.insert("foobar", 1);
        t.insert("foobarbaz", 2);
        let foo = t.root.as_ref().unwrap();
        assert!(foo.key == b"foo");
        assert!(foo.value == Some(0));
        assert!(foo.sibling.is_none());
        let bar = foo.child.as_ref().unwrap();
        assert!(bar.key == b"bar");
        assert!(bar.value == Some(1));
        assert!(bar.sibling.is_none());
        let baz = bar.child.as_ref().unwrap();
        assert!(baz.key == b"baz");
        assert!(baz.value == Some(2));
        assert!(baz.child.is_none());
        assert!(baz.sibling.is_none());
    }

    #[test]
    fn test_insert_sibling() {
        let mut t = Tree::new();
        t.insert("foo", 0);
        t.insert("bar", 1);
        t.insert("quux", 2);
        let foo = t.root.as_ref().unwrap();
        assert!(foo.key == b"foo");
        assert!(foo.value == Some(0));
        assert!(foo.child.is_none());
        let bar = foo.sibling.as_ref().unwrap();
        assert!(bar.key == b"bar");
        assert!(bar.value == Some(1));
        assert!(bar.child.is_none());
        let quux = bar.sibling.as_ref().unwrap();
        assert!(quux.key == b"quux");
        assert!(quux.value == Some(2));
        assert!(quux.child.is_none());
        assert!(quux.sibling.is_none());
    }

    #[test]
    fn test_insert_split() {
        let mut t = Tree::new();
        t.insert("foo", 0);
        t.insert("fobar", 1);
        let root = t.root.as_ref().unwrap();
        assert!(root.key == b"fo");
        assert!(root.value.is_none());
        assert!(root.sibling.is_none());
        let foo = root.child.as_ref().unwrap();
        assert!(foo.key == b"o");
        assert!(foo.value == Some(0));
        assert!(foo.child.is_none());
        let bar = foo.sibling.as_ref().unwrap();
        assert!(bar.key == b"bar");
        assert!(bar.value == Some(1));
        assert!(bar.sibling.is_none());
        assert!(bar.child.is_none());
    }
}
