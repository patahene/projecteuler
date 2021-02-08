fn main() {
    let mut s = 0;
    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            s += n
        }
    }
    println!("{}", s);
}
