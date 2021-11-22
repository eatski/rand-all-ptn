use std::{collections::HashMap, hash::Hash};

#[derive(Eq,PartialEq,Debug)]
pub enum Tree<K: Eq + Hash + Copy,V> {
    Leaf(V),
    Node(HashMap<K,Tree<K,V>>),
}

impl<K: Eq + Hash + Copy,V>Tree<K,V> {
    pub fn set(&mut self,target: Self) {
        *self = target;
    }
    pub fn get_deep_mut<I: Iterator<Item = K>>(&mut self,iter: I) -> &mut Self {
        iter.fold(self, |acc,cur| {
            acc.get_mut(cur).unwrap()
        })
    }
    pub fn get_mut(&mut self,key: K) -> Option<&mut Self>{
        match self {
            Tree::Leaf(_) => None,
            Tree::Node(map) => map.get_mut(&key),
        }
    }
}

#[test]
fn tree_set() {
    let mut tree: Tree<usize,&str> = Tree::Node(HashMap::new());
    tree.set(Tree::Leaf("hoge"));
    assert_eq!(tree,Tree::Leaf("hoge"));
}

#[test]
fn tree_get() {
    let mut origin: Tree<usize,&str> = Tree::Node(
        [
            (0,Tree::Node(
                [
                    (1,Tree::Leaf("fuga"))
                ].into()
            )),
        ].into()
    );
    {
        let tree = origin.get_deep_mut([0,1].into_iter());
        assert_eq!(tree,&Tree::Leaf("fuga"));
        tree.set(Tree::Node([(0,Tree::Leaf("piyo"))].into()));
        let tree = tree.get_deep_mut([0].into_iter());
        assert_eq!(tree,&Tree::Leaf("piyo"));
    }
    let tree = origin.get_deep_mut([0,1,0].into_iter());
    assert_eq!(tree,&Tree::Leaf("piyo"));
}


