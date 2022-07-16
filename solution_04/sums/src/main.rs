fn sum(nums: &[u32]) -> Option<u32> {
    nums.iter().try_fold(0u32, |left, &right| left.checked_add(right))
}

fn print_sum(nums: &[u32]) {
    match sum(&nums) {
        Some(v) => println!("sum is {}", v),
        None => println!("overflow!!"),
    }
}

fn main() {
    print_sum(&[999, 99, 9999]);
    print_sum(&[999, 99, u32::MAX]);
}