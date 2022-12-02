use rlib_dsu::*;

#[test]
fn simple() {
    let mut dsu = DSU::new(4);
    assert!(dsu.un(0, 1));
    let root = dsu.par(0);
    assert!(root == 0 || root == 1);

    assert!(dsu.un(1, 2));
    assert_eq!(dsu.par(0), root);
    assert_eq!(dsu.par(1), root);
    assert_eq!(dsu.par(2), root);
    assert_eq!(dsu.par(3), 3);
    assert_eq!(dsu.size(0), 3);
    assert_eq!(dsu.size(1), 3);
    assert_eq!(dsu.size(2), 3);
    assert!(dsu.check(0, 1));
    assert!(dsu.check(0, 2));
    assert!(dsu.check(2, 1));
    assert!(!dsu.check(3, 0));
    assert!(!dsu.check(1, 3));

    assert!(dsu.un(3, 0));
    assert_eq!(dsu.par(0), root);
    assert_eq!(dsu.par(1), root);
    assert_eq!(dsu.par(2), root);
    assert_eq!(dsu.par(3), root);
    assert_eq!(dsu.size(0), 4);
    assert_eq!(dsu.size(1), 4);
    assert_eq!(dsu.size(2), 4);
    assert_eq!(dsu.size(3), 4);

    dsu.reset(3);
    for i in 0..3 {
        assert_eq!(dsu.par(i), i);
        assert_eq!(dsu.size(i), 1);
    }

    dsu.un(0, 1);
    dsu.un(1, 2);

    dsu.reset(10);
    for i in 0..10 {
        assert_eq!(dsu.par(i), i);
        assert_eq!(dsu.size(i), 1);
    }
}
