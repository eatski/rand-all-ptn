use std::{collections::HashMap, hash::Hash, iter::{repeat, successors}, ops::{Add, Range, Sub}, panic, process::Output};

use tree::Tree;
mod tree;

#[test]
fn it_works() {
    run(|rnd| {
        for _ in 0..10 {
            rnd.gen_range(0..3);
        }
    },1);
}

#[test]
fn it_works_large() {
    run(|rnd| {
        for _ in 0..100 {
            rnd.gen_range(0..10);
        }
    },1);
}

#[test]
fn is_closed() {
    let mut tree : RandmanTree<usize> = Tree::Leaf(EmptyOrClose::Close);
    assert!(RandmanAll::is_close(&mut tree));
    let mut tree : RandmanTree<usize> = Tree::Node([
        (0,Tree::Leaf(EmptyOrClose::Close)),
        (1,Tree::Leaf(EmptyOrClose::Close)),
        (2,Tree::Leaf(EmptyOrClose::Close)),
    ].into());
    assert!(RandmanAll::is_close(&mut tree));
    assert_eq!(tree,Tree::Leaf(EmptyOrClose::Close));
}

#[test]
fn gen_range() {
    let mut rand = RandmanAll::new(1);
    let res = (rand.gen_range(0..2),rand.gen_range(0..2));
    assert_eq!(res,(0,0));
    rand.init();
    assert_eq!(rand.tree,
        Tree::Node([
            (0,Tree::Node(
                [
                    (0,Tree::Leaf(EmptyOrClose::Close)),
                    (1,Tree::Leaf(EmptyOrClose::Empty)),
                ].into()
            )),
            (1,Tree::Leaf(EmptyOrClose::Empty))
        ].into()
    ));
    let res = (rand.gen_range(0..2),rand.gen_range(0..2));
    assert_eq!(res,(0,1));
    rand.init();
    let res = (rand.gen_range(0..2),rand.gen_range(0..2));
    assert_eq!(res,(1,0));
    rand.init();
    let res = (rand.gen_range(0..2),rand.gen_range(0..2));
    assert_eq!(res,(1,1));
    rand.init();
    assert!(RandmanAll::is_close(&mut rand.tree));
}

#[test]
fn test_range_to_vec() {
    let vec = range_to_vec(1..3,1);
    let exp = (1..3).collect::<Vec<i32>>();
    assert_eq!(vec,exp)
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
    tree:RandmanTree<T>,
    stack: Vec<T>
}

type RandmanTree<T> = Tree<T,EmptyOrClose>;

fn range_to_vec<T: Add<Output = T> + Eq + Copy>(range: Range<T>,step: T) -> Vec<T>{
    successors(Some(range.start), |val| {if *val + step == range.end { None } else { Some(*val + step) }}).collect()
}

impl <T: Add<Output = T> + Copy + PartialEq + Eq + Hash>Randaman<T> for RandmanAll<T> {
    fn gen_range(&mut self,range: Range<T>) -> T {
        let range_vec : Vec<_> = range_to_vec(range,self.step);
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
                    !Self::is_close(next)
                },
                None => {
                    let hashmap: HashMap<_,_> = range_vec
                        .iter()
                        .map(|e| (*e,Tree::Leaf(EmptyOrClose::Empty)))
                        .collect();
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

#[derive(Debug,PartialEq, Eq)]
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
    fn is_close(tree: &mut RandmanTree<T>) -> bool {
        match tree {
            Tree::Leaf(EmptyOrClose::Empty) => false,
            Tree::Leaf(EmptyOrClose::Close) => true,
            Tree::Node(map) => {
                let closed = map.iter_mut().all(|(_,tree)| Self::is_close(tree));
                if closed {
                    tree.set(Tree::Leaf(EmptyOrClose::Close));
                }
                closed
            },
        }
    }
}