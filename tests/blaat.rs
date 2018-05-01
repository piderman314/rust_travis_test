extern crate travis_test;

#[test]
fn itest() {
    assert_eq!(5, travis_test::add(2, 3));
}