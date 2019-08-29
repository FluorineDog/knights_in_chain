
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

fn main() {
    let fibo_vec = gen_fibo(10);
    println!("{:?}", fibo_vec);
    // println!("Hello, world!");
}
