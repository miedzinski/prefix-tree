fn common_prefix<T: Eq>(a: &[T], b: &[T]) -> usize {
    a.iter().zip(b).take_while(|&(a, b)| a == b).count()
}

#[derive(Debug, Clone)]
pub struct Tree<K, V> {
    key: Vec<K>,
    pub value: Option<V>,
    child: Option<Box<Tree<K, V>>>,
    sibling: Option<Box<Tree<K, V>>>,
}

impl<K: Eq + Clone, V> Tree<K, V> {
    pub fn new(key: Vec<K>, value: V) -> Tree<K, V> {
        Tree {
            key,
            value: Some(value),
            child: None,
            sibling: None,
        }
    }

    pub fn common_prefix(&self, other: &[K]) -> usize {
        self.key
            .iter()
            .zip(other.as_ref().iter())
            .take_while(|&(a, b)| a == b)
            .count()
    }

    pub fn find(&self, key: &[K]) -> Option<&Tree<K, V>> {
        if key.is_empty() && self.key.is_empty() {
            return Some(self);
        }
        match common_prefix(&self.key, key) {
            0 => self.sibling.as_ref().and_then(|x| x.find(key)),
            p if p == self.key.len() => {
                if p == key.len() {
                    Some(self)
                } else {
                    self.child.as_ref().and_then(|x| x.find(&key[p..]))
                }
            }
            _ => None,
        }
    }

    pub fn find_mut(&mut self, key: &[K]) -> Option<&mut Tree<K, V>> {
        if key.is_empty() && self.key.is_empty() {
            return Some(self);
        }
        match self.common_prefix(key) {
            0 => self.sibling.as_mut().and_then(|x| x.find_mut(key)),
            p if p == self.key.len() => {
                if p == key.len() {
                    Some(self)
                } else {
                    self.child.as_mut().and_then(|x| x.find_mut(&key[p..]))
                }
            }
            _ => None,
        }
    }

    pub fn insert(&mut self, key: &[K], value: V) {
        let prefix = self.common_prefix(key);
        if prefix == 0 {
            if let Some(ref mut sibling) = self.sibling {
                sibling.insert(key, value);
            } else {
                self.sibling = Some(Box::new(Tree::new(key.to_vec(), value)));
            }
        } else {
            if prefix < self.key.len() {
                self.child = Some(Box::new(Tree {
                    key: self.key.split_off(prefix),
                    value: self.value.take(),
                    child: self.child.take(),
                    sibling: None,
                }));
            }
            if prefix < key.len() {
                if let Some(ref mut child) = self.child {
                    child.insert(&key[prefix..], value);
                } else {
                    self.child = Some(Box::new(Tree::new(key[prefix..].to_vec(), value)));
                }
            } else {
                self.value = Some(value);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_common_prefix() {
        assert_eq!(common_prefix(&[1, 2, 3], &[]), 0);
        assert_eq!(common_prefix(&[1, 2, 3, 4, 5, 6], &[1, 2, 9, 4, 5, 6]), 2);
    }

    fn sample_tree() -> Tree<i32, u8> {
        Tree {
            key: vec![1, 2],
            value: Some(0),
            child: Some(Box::new(Tree {
                key: vec![3],
                value: Some(1),
                child: None,
                sibling: Some(Box::new(Tree::new(vec![-3], 2))),
            })),
            sibling: Some(Box::new(Tree::new(vec![9, 8, 7], 3))),
        }
    }

    #[test]
    fn test_find() {
        let t = sample_tree();
        assert_eq!(t.find(&[1, 2]).and_then(|x| x.value), Some(0));
        assert_eq!(t.find(&[1, 2, 3]).and_then(|x| x.value), Some(1));
        assert_eq!(t.find(&[9, 8, 7]).and_then(|x| x.value), Some(3));
        assert!(t.find(&[]).is_none());
        assert!(t.find(&[4, 5, 6]).is_none());
        assert!(t.find(&[0]).is_none());
        assert!(t.find(&[1, 2, 3, 3]).is_none());
    }

    #[test]
    fn test_find_mut() {
        assert_eq!(
            sample_tree().find_mut(&[1, 2]).and_then(|x| x.value),
            Some(0)
        );
        assert_eq!(
            sample_tree().find_mut(&[1, 2, 3]).and_then(|x| x.value),
            Some(1)
        );
        assert_eq!(
            sample_tree().find_mut(&[9, 8, 7]).and_then(|x| x.value),
            Some(3)
        );
        assert!(sample_tree().find_mut(&[4, 5, 6]).is_none());
        assert!(sample_tree().find_mut(&[0]).is_none());
        assert!(sample_tree().find_mut(&[1, 2, 3, 3]).is_none());
    }

    #[test]
    fn test_insert_append() {
        let mut root = Tree::new(vec![1, 2, 3], 0);
        root.insert(&[], 999);
        root.insert(&[1], 2);
        root.insert(&[1, 2, 3, 4, 5, 6], 1);
        root.insert(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 2);
        root.insert(&[3, 2, 1], -1);
        root.insert(&[1, 2, 5, 6], 9);

        assert_eq!(root.find(&[]).and_then(|x| x.value), Some(999));
        assert_eq!(root.find(&[1]).and_then(|x| x.value), Some(2));
        assert_eq!(
            root.find(&[1, 2, 3, 4, 5, 6]).and_then(|x| x.value),
            Some(1)
        );
        assert_eq!(
            root.find(&[1, 2, 3, 4, 5, 6, 7, 8, 9])
                .and_then(|x| x.value),
            Some(2)
        );
        assert_eq!(root.find(&[3, 2, 1]).and_then(|x| x.value), Some(-1));
        assert_eq!(root.find(&[1, 2, 5, 6]).and_then(|x| x.value), Some(9));
    }
}
