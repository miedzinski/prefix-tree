use std::mem;

fn common_prefix<T: Eq>(a: &[T], b: &[T]) -> usize {
    a.iter().zip(b).take_while(|&(a, b)| a == b).count()
}

#[derive(Debug, Clone, Default)]
pub struct Tree<K, V> {
    key: Vec<K>,
    pub value: Option<V>,
    children: Vec<Tree<K, V>>,
}

impl<K: Eq + Clone, V> Tree<K, V> {
    pub fn new(key: Vec<K>, value: V) -> Tree<K, V> {
        Tree {
            key,
            value: Some(value),
            children: vec![],
        }
    }

    pub fn empty() -> Tree<K, V> {
        Tree {
            key: vec![],
            value: None,
            children: vec![],
        }
    }

    pub fn find(&self, key: &[K]) -> Option<&Tree<K, V>> {
        let p = common_prefix(&self.key, key);
        if p != self.key.len() {
            return None;
        }
        if p < key.len() {
            self.children
                .iter()
                .map(|x| x.find(&key[p..]))
                .filter_map(|x| x)
                .next()
        } else if self.value.is_some() {
            Some(self)
        } else {
            None
        }
    }

    pub fn find_mut(&mut self, key: &[K]) -> Option<&mut Tree<K, V>> {
        let p = common_prefix(&self.key, key);
        if p != self.key.len() {
            return None;
        }
        if p < key.len() {
            self.children
                .iter_mut()
                .map(|x| x.find_mut(&key[p..]))
                .filter_map(|x| x)
                .next()
        } else if self.value.is_some() {
            Some(self)
        } else {
            None
        }
    }

    pub fn insert(&mut self, key: &[K], value: V) -> Option<V> {
        let p = common_prefix(&self.key, key);
        if p < self.key.len() {
            let child = Tree {
                key: self.key.split_off(p),
                value: self.value.take(),
                children: mem::take(&mut self.children),
            };
            self.children.push(child);
        }
        if p == key.len() {
            self.value.replace(value)
        } else {
            let mut child = self
                .children
                .iter_mut()
                .find(|x| common_prefix(&x.key, &key[p..]) > 0);
            if let Some(ref mut child) = child {
                child.insert(&key[p..], value)
            } else {
                self.children.push(Tree::new(key[p..].to_vec(), value));
                None
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
            key: vec![],
            value: None,
            children: vec![
                Tree {
                    key: vec![1, 2],
                    value: Some(0),
                    children: vec![Tree::new(vec![3], 1), Tree::new(vec![-3], 2)],
                },
                Tree::new(vec![9, 8, 7], 3),
            ],
        }
    }

    #[test]
    fn test_find() {
        let t = sample_tree();
        assert_eq!(t.find(&[1, 2]).and_then(|x| x.value), Some(0));
        assert_eq!(t.find(&[1, 2, 3]).and_then(|x| x.value), Some(1));
        assert_eq!(t.find(&[9, 8, 7]).and_then(|x| x.value), Some(3));
        assert!(dbg!(t.find(&[])).is_none());
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
    fn test_insert() {
        let mut root = Tree::new(vec![1, 2, 3], 0);
        root.insert(&[3, 2, 1], -1);
        root.insert(&[], 999);
        root.insert(&[1], 2);
        root.insert(&[1, 2, 3, 4, 5, 6], 1);
        root.insert(&[1, 2, 3, 4, 5, 6, 7, 8, 9], 2);
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
