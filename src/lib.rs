use std::{collections::HashMap, hash::Hash, iter::{repeat, successors}, ops::{Add, Range}, panic};

use tree::Tree;
mod tree;

#[test]
fn it_works() {
    run(|rnd| {
        testee(rnd);
    },1);
}

fn testee<R: Randaman<i32>>(randman: &mut R){
    for _ in 1..1000 {
        randman.gen_range(1..10);
    }
}

#[test]
fn gen_range() {
    let mut rand = RandmanAll::new(1);
    let res = (rand.gen_range(1..2),rand.gen_range(1..2));
    assert_eq!(res,(1,1));
    rand.init();
    let res = (rand.gen_range(1..2),rand.gen_range(1..2));
    assert_eq!(res,(1,2));
    rand.init();
    let res = (rand.gen_range(1..2),rand.gen_range(1..2));
    assert_eq!(res,(2,1));
    rand.init();
    let res = (rand.gen_range(1..2),rand.gen_range(1..2));
    assert_eq!(res,(2,2));
    assert!(RandmanAll::is_close(&mut rand.tree));
}

pub fn run<T:Add<Output = T> + Copy + PartialEq + Eq + Hash,F: FnMut(&mut RandmanAll<T>)>(mut test_fn: F,range_step:T) {
    let mut rand = RandmanAll::new(range_step);
    repeat(()).any(|_| {
        test_fn(&mut rand);
        rand.init();
        RandmanAll::is_close(&mut rand.tree)
    });
}

pub trait Randaman<T: Add<Output = T> + Copy + PartialEq + Eq + Hash> {
    fn gen_range(&mut self,range: Range<T>) -> T;
}

pub struct RandmanAll<T: Add<Output = T> + Copy + PartialEq + Eq + Hash> {
    step: T,
    tree:Tree<T,EmptyOrClose>,
    stack: Vec<T>
}

impl <T: Add<Output = T> + Copy + PartialEq + Eq + Hash>Randaman<T> for RandmanAll<T> {
    fn gen_range(&mut self,range: Range<T>) -> T {
        let range_vec : Vec<_> = successors(Some(range.start), |val| {if *val == range.end { None } else { Some(*val + self.step) }}).collect();
        let found = range_vec.iter().find(|idx| {
            let current = self.tree.get_deep_mut(self.stack.iter().map(Clone::clone));
            let map = match current {
                Tree::Leaf(EmptyOrClose::Empty) => {
                    None
                },
                Tree::Leaf(EmptyOrClose::Close) => panic!(),
                Tree::Node(map) => {
                    Some(map)
                },
            };
            match map {
                Some(map) => {
                    let next = map.get_mut(*idx).expect("初期化時に作られてるはず");
                    Self::is_close(next)
                },
                None => {
                    let hashmap: HashMap<_,_> = range_vec.iter().map(|e| (*e,Tree::Leaf(EmptyOrClose::Empty))).collect();
                    current.set(Tree::Node(hashmap));
                    true
                },
            }
        });
        let result = *found.expect("Close Faulure");
        self.stack.push(result);
        result
    }
}

enum EmptyOrClose {
    Empty,Close
}
impl <T: Add<Output = T> + Copy + PartialEq + Eq + Hash>RandmanAll<T> {
    fn new(step:T) -> Self {
        Self {
            step,
            tree: Tree::Leaf(EmptyOrClose::Empty),
            stack: Vec::new()
        }
    }
    fn init(&mut self) {
        let last = self.tree.get_deep_mut(self.stack.iter().map(Clone::clone));
        last.set(Tree::Leaf(EmptyOrClose::Close));
        self.stack = Vec::new();
    }
    fn is_close(tree: &mut Tree<T,EmptyOrClose>) -> bool {
        match tree {
            Tree::Leaf(EmptyOrClose::Empty) => true,
            Tree::Leaf(EmptyOrClose::Close) => false,
            Tree::Node(map) => {
                let closed = map.iter().all(|(_,tree)| matches!(tree,Tree::Leaf(EmptyOrClose::Close)));
                if closed {
                    tree.set(Tree::Leaf(EmptyOrClose::Close));
                }
                !closed
            },
        }
    }
}