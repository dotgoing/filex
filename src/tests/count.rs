fn main() {
    println!("this count test");
}

#[test]
fn test_sub() {
    assert_eq!(3, sub(5, 2));
    assert_eq!(4, sub(6, 2));
}