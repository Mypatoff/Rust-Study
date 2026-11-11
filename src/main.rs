fn main() {
    let mut a = 27;
    let mut steps = 0;
    let mut highest = 0;
    loop {
        if highest < a {
            highest = a;
        }
        println!("a: {}", a);
        a = pr(a);
        steps = steps + 1;
        if a == 1 {
            println!("a: {}", a);
            break;
        }
    }
    println!("{}", steps);
    println!("Highest: {}", highest);
}
fn pr(a: i32) -> i32 {
    if a % 2 == 0 { a / 2 } else { a * 3 + 1 }
}
