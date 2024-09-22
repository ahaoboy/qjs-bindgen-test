fn main() {
    let mut args = std::env::args();
    args.next();
    if let (Some(a), Some(b)) = (args.next(), args.next()) {
        let a: i32 = a.parse().unwrap();
        let b: i32 = b.parse().unwrap();
        let c = core::add(a, b);
        println!("{}+{}={}", a, b, c);
    }
}
