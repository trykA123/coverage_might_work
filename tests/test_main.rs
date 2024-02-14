use coverage_might_work::main;

#[test]
fn test_add() {
    assert_eq!(add(2, 3), 5);
}

#[test]
fn test_add2() {
    assert_eq!(add(4, 3), 7);
}

