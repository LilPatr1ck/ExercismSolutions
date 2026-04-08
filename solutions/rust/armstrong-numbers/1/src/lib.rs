pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 { return true }
    let mut sum: u32 = 0;
    let s = num.to_string();
    let power: u32 = s.len() as u32;
    for ch in s.chars() {
        let digit = ch.to_digit(10).unwrap();
        sum += digit.pow(power);
    }
    sum == num
}
