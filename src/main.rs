const FIN: usize = 83;

fn gen_fibo(n: usize) -> Vec<i64> {
    let mut res = Vec::new();
    assert!(n > 2);
    let mut r0 = 0;
    let mut r1 = 1;
    for _i in 0..n {
        res.push(r0);
        let (r0_, r1_) = (r1, r0 + r1);
        r0 = r0_;
        r1 = r1_;
    }
    res
}

thread_local!(static TABLE: Vec<i64> = gen_fibo(FIN));

struct Meta {
    a: i64,
    b: i64,
    c: i64,
    loc: usize,
    value: i64,
}

impl Meta{
fn upgrade(&this) -> Meta {
    this
}
}

fn upgrade_to_top(old: Meta) -> Meta {
    old
}


fn main() {
    let fibo_vec = gen_fibo(FIN);
    println!("{:?}", fibo_vec);
    // println!("Hello, world!");
}
