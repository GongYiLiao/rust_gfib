use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::boxed::Box;
use std::ops::Fn;
use std::vec::Vec;

struct Gfib<T> {
    k: usize,
    gf: Box<dyn Fn(&Vec<T>) -> T>,
    xs: Vec<T>,
}

impl<T> Gfib<T>
where
    T: Clone,
{
    fn update(&mut self) {
        let n = self.xs.len();
        let m: usize = n - self.k;
        let last_k_xs = self.xs.as_slice()[m..].to_vec();
        let x_new = (self.gf)(&last_k_xs);
        self.xs.push(x_new.clone());
    }

    fn new(xs_init: Vec<T>, gf_tgt: Box<dyn Fn(&Vec<T>) -> T>) -> Self {
        let k_val = xs_init.len();
        if k_val == 0 {
            panic!("Initial XS must contain at least one value");
        } else {
            Self {
                k: k_val,
                gf: gf_tgt,
                xs: xs_init,
            }
        }
    }

    fn take_n(&mut self, n: usize) -> Vec<T> {
        let n_now = self.xs.len();
        if n > n_now {
            let m = n - n_now;
            for _i in 0..m {
                self.update();
            }
        }
        self.xs.as_slice()[0..n].to_vec()
    }

    fn nth(&mut self, n: usize) -> T {
        self.take_n(n + 1).last().cloned().unwrap()
    }
}

fn test_gfib_0() {
    let bui_sum = |xs: &Vec<BigUint>| -> BigUint { xs.iter().sum::<BigUint>() };
    let mut _gfib_0 =
        Gfib::<BigUint>::new(vec![BigUint::zero(), BigUint::one()], Box::new(bui_sum));
    let mut _gfib_1 = Gfib::<BigUint>::new(
        vec![BigUint::zero(), BigUint::zero(), BigUint::one()],
        Box::new(bui_sum),
    );
    for _i in 0..31 {
        println!(
            "Fib(0, 1)[{}] = {};  Fib(0, 0, 1)[{}] = {}",
            _i,
            _gfib_0.nth(_i),
            _i,
            _gfib_1.nth(_i)
        );
    }
    println!("Fib(0, 1)[200] = {}", _gfib_0.nth(200));
    println!("Fib(0, 0, 1)[200] = {}", _gfib_1.nth(200));
}

fn main() {
    test_gfib_0();
}
