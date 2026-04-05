pub fn n(n: u32) {
    assert!((2..150_001).contains(&n));
}

pub fn m(m: u32) {
    assert!((1..150_001).contains(&m));
}

pub fn fountain(fountain: u32) {
    assert!(fountain < 150_000);
}

pub fn state(state: u32) {
    assert!(state < 300_000);
}

pub fn steps(steps: u32) {
    assert!(steps < 300_001);
}
