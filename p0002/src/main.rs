fn main() {
    let mut t1 = 1;
    let mut t2 = 2;
    let mut s = t2;
    loop {
        let t3 = t1 + t2;
        if t3 > 4_000_000 {
            break;
        }
        t1 = t2;
        t2 = t3;
        if t3 % 2 == 0 {
            s += t3;
        }
    }
    println!("{}", s);
}
