#[macro_use]
extern crate lazy_static;

const FIN: usize = 83;
fn gen_fibo(n: usize) -> Vec<i64> {
    let mut res = Vec::new();
    assert!(n > 2);
    let mut r0 = 0;
    let mut r1 = 1;
    for _i in 0..(n + 1) {
        res.push(r0);
        let (r0_, r1_) = (r1, r0 + r1);
        r0 = r0_;
        r1 = r1_;
    }
    res
}

lazy_static! {
    static ref TABLE: Vec<i64> = gen_fibo(FIN);
}

#[derive(Copy, Clone)]
struct Meta {
    r0: i64, // pair sum = fib(n-1), left side
    r1: i64, // pair sum = fib(n), left side
    r2: i64, // pair sum = fib(n-1), left side
    level: usize,
    value: i64,
}

fn fibo(level: usize) -> i64 {
    TABLE[level]
}

impl Meta {
    fn location(&self) -> usize {
        (self.r0 + self.r1 + self.r2) as usize
    }
}

impl Meta {
    fn upgrade(&self) -> Meta {
        let Meta {
            r0,
            r1,
            r2,
            level,
            value,
        } = self.clone();

        Meta {
            r0: r1,
            r1: r0 * 2 + r1,
            r2: r2 + r0 + 1,
            level: level + 1,
            value,
        }
    }
}

impl Meta {
    fn upgrade_leftmost(&self) -> Meta {
        assert!(self.value == fibo(self.level));
        assert!(self.location() == 0);
        let level = self.level + 1;
        let value = fibo(level);
        Meta {
            r0: 0,
            r1: 0,
            r2: 0,
            level,
            value,
        }
    }
}

impl Meta {
    fn upgrade_rightmost(&self) -> (Meta, bool) {
        let Meta {
            r0,
            r1,
            r2,
            level,
            value,
        } = self.clone();
        let flag = (fibo(level + 1) - fibo(level)) % 2 == 0;
        if flag {
            (self.clone(), flag)
        } else {
            (self.clone(), flag)
        }
    }
}

fn range_upscale(left: &Meta, right: &Meta) -> Option<(Meta, Meta)> {
    Some((left.clone(), right.clone()))
}

fn main() {
    let fibo_vec = gen_fibo(FIN);
    println!("{:?}", fibo_vec);
    // println!("Hello, world!");
}
