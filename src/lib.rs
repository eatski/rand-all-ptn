use std::{hash::Hash, iter::{repeat}, ops::{Add, Range}};
mod sugoroku;

#[test]
fn it_works() {
    let range = 0..6;
    let num = 0..10;
    let couter = run(|rnd| {
        for _ in num.clone() {
            rnd.gen_range(range.clone());
        }
    },1);
    assert_eq!(couter,range.len().pow(num.len().try_into().unwrap()));
}

#[test]
fn gen_range() {
    let mut rand = RandmanAll::new(1);
    let res = (rand.gen_range(0..2),rand.gen_range(0..2));
    assert_eq!(res,(0,0));
    assert_eq!(rand.init(),true);
    let res = (rand.gen_range(0..2),rand.gen_range(0..2));
    assert_eq!(res,(0,1));
    assert_eq!(rand.init(),true);
    let res = (rand.gen_range(0..2),rand.gen_range(0..2));
    assert_eq!(res,(1,0));
    assert_eq!(rand.init(),true);
    let res = (rand.gen_range(0..2),rand.gen_range(0..2));
    assert_eq!(res,(1,1));
    assert_eq!(rand.init(),false);
}

#[test]
fn sugoroku() {
    let counter = run(sugoroku::sugoroku::run,1);
    assert_eq!(counter,2358561);
}

pub fn run<T:Add<Output = T> + Copy + PartialEq + Eq + Hash,F: FnMut(&mut RandmanAll<T>)>(mut test_fn: F,range_step:T) -> usize{
    let mut rand = RandmanAll::new(range_step);
    repeat(()).take_while(|_| {
        test_fn(&mut rand);
        rand.init()
    }).count() + 1
}

pub trait Randaman<T: Add<Output = T> + Copy + PartialEq + Eq + Hash> {
    fn gen_range(&mut self,range: Range<T>) -> T;
}

pub struct RandmanAll<T: Add<Output = T> + Copy + PartialEq + Eq + Hash> {
    step: T,
    stack: Stack<T>,
}

impl <T: Add<Output = T> + Copy + PartialEq + Eq + Hash>Randaman<T> for RandmanAll<T> {
    fn gen_range(&mut self,range: Range<T>) -> T {
        let current = self
            .stack
            .history
            .get(self.stack.position);
        let result = match current {
            Some(current) => *current,
            None => {
                let start = range.start;
                self.stack.history.push(start);
                start
            },
        };
        // endフラグを付与
        if result + self.step == range.end {
            if  self.stack.end.is_none() {
                self.stack.end = Some(self.stack.position);
            }
        } else {
            self.stack.end = None
        }
        self.stack.position = self.stack.position + 1;
        result
    }
}


impl <T: Add<Output = T> + Copy + PartialEq + Eq + Hash>RandmanAll<T> {
    fn new(step:T) -> Self {
        Self {
            step,
            stack: Stack::new(),
        }
    }
    fn init(&mut self) -> bool {
        if self.stack.history.len() == 0 {
            return false
        }
        let continu = match self.stack.end {
            Some(0) => {
                false
            }
            Some(end) => {
                self.stack.history = self.stack.history.drain(0..end).collect();
                true
            },
            None => true,
        };
        if continu {
            let last = self.stack.history.last_mut().unwrap();
            *last = *last + self.step;
            self.stack.position = 0;
            self.stack.end = None;
        }
        continu
    }
}

pub struct Stack<T> {
    history: Vec<T>,
    end: Option<usize>,
    position: usize
}

impl <T>Stack<T> {
    pub fn new() -> Self {
        Self {
            history: Vec::new(),
            end: None,
            position: 0
        }
    }
}
