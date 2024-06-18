pub mod div;

fn main() {
    let mut a = String::new();
    let mut b = String::new();
    println!("Enter num1");
    std::io::stdin().read_line(&mut a).unwrap();
    println!("Enter num2");
    std::io::stdin().read_line(&mut b).unwrap();
    let c = a.trim().parse::<i8>();
    let d = b.trim().parse::<i8>();

    println!("the awnser is {}", div::eve(c.expect("err"),d.expect("err")))
}