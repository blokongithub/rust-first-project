pub fn add() {
    let nums = getoptions();
    println!("The awnser is {}", nums[0] + nums[1]);
}
pub fn subtract() {
    let nums = getoptions();
    println!("The awnser is {}", nums[0] - nums[1]);
}
pub fn multiply() {
    let nums = getoptions();
    println!("The awnser is {}", nums[0] * nums[1]);
}

pub fn divide() {
    let nums = getoptions();
    println!("The awnser is {}", nums[0] as f32 / nums[1] as f32);
}

pub fn getoptions() -> [i32; 2] {
    let mut num1 = String::new();
    let mut num2 = String::new();
    println!("enter num 1");
    std::io::stdin().read_line(&mut num1).unwrap();
    println!("enter num 2");
    std::io::stdin().read_line(&mut num2).unwrap();
    let x = num1.trim().parse::<i32>().unwrap();
    let y = num2.trim().parse::<i32>().unwrap();
    return [x, y];
}