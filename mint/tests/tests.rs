use rlib_mint::*;

#[test]
fn small_mod() {
    type Mint = Modular<5>;

    assert_eq!(Mint::new(10), Mint::ZERO);
    assert_eq!(Mint::new(5), Mint::ZERO);
    assert_eq!(Mint::new(0), Mint::ZERO);
    assert_eq!(Mint::new(-5), Mint::ZERO);
    assert_eq!(Mint::new(-10), Mint::ZERO);

    assert_eq!(Mint::new(1), Mint::ONE);
    assert_eq!(Mint::new(6), Mint::ONE);
    assert_eq!(Mint::new(-4), Mint::ONE);
    assert_eq!(Mint::new(-9), Mint::ONE);

    assert_eq!(Mint::new(4), Mint::new(4));
    assert_eq!(Mint::new(9), Mint::new(4));
    assert_eq!(Mint::new(-1), Mint::new(4));
    assert_eq!(Mint::new(-6), Mint::new(4));
}

#[test]
fn operators() {
    type Mint = Modular<11>;

    assert_eq!(Mint::new(5) + Mint::new(6), Mint::ZERO);
    assert_eq!(Mint::new(5) + Mint::new(10), Mint::new(4));

    assert_eq!(Mint::new(5) - Mint::new(6), -Mint::ONE);
    assert_eq!(Mint::new(5) - Mint::new(10), Mint::new(6));
    assert_eq!(Mint::new(5) - Mint::new(5), Mint::ZERO);

    assert_eq!(-Mint::new(0), Mint::ZERO);
    assert_eq!(-Mint::new(5), Mint::new(6));

    assert_eq!(Mint::new(5) * Mint::new(6), Mint::new(8));
    assert_eq!(Mint::new(5) * Mint::new(0), Mint::new(0));

    assert_eq!(Mint::new(5) / Mint::new(6), Mint::new(10));
    assert_eq!(Mint::new(5) / Mint::new(-1), Mint::new(-5));

    for a in -30..=30 {
        for b in -30..=30 {
            let x = Mint::new(a);
            let y = Mint::new(b);

            assert_eq!(x + y, Mint::new(a + b));
            assert_eq!(x - y, Mint::new(a - b));
            assert_eq!(x * y, Mint::new(a * b));

            let mut x = Mint::new(a);
            x += y;
            assert_eq!(x, Mint::new(a + b));
            let mut x = Mint::new(a);
            x -= y;
            assert_eq!(x, Mint::new(a - b));
            let mut x = Mint::new(a);
            x *= y;
            assert_eq!(x, Mint::new(a * b));
        }

        for b in 1..Mint::md() as i64 {
            let x = Mint::new(a);
            let z = x / Mint::new(b);
            assert_eq!(z * Mint::new(b), x);

            let mut x = Mint::new(a);
            x /= Mint::new(b);
            assert_eq!(x * Mint::new(b), Mint::new(a));
        }
    }
}

#[test]
fn common_mods() {
    assert_eq!(Mint998::md(), 998_244_353);
    assert_eq!(Mint107::md(), 1_000_000_007);
}
