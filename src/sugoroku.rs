pub (crate) mod sugoroku {
    use std::{iter::{repeat}};

    use crate::Randaman;

    pub (crate) fn run<R: Randaman<i32>>(randaman : &mut R) {
        repeat(()).scan(0, |state,_| {
            *state = *state + randaman.gen_range(1..7);
            if *state < 20 {
                Some(())
            } else {
                None
            }
        }).count();
    }
}