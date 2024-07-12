fn main() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);

    let (_a, _b, c) = tup;

    println!("The value of c is: {}", c);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    println!("{} {} {}", five_hundred, six_point_four, one);

    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
