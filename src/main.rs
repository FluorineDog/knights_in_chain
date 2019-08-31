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

#[derive(Copy, Clone)]
struct Meta {
    a: i64,
    b: i64,
    c: i64, 
    loc: usize,
    value: i64,
}

impl Meta{
fn upgrade(&self) -> Meta {
    self.clone()
}
}

impl Meta {
fn upgrade_plus(&self) -> Meta{
    self.clone()
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
