extern crate tests;

#[test]
pub fn test_all() {
    assert_eq!(tests::add(3, 5), 8);
    assert_eq!(tests::test_lib::sub_numbers(8, 3), 5);
}