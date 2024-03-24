#[allow(dead_code)]
fn find_midpoint(low: u32, high: u32) -> u32 {
    return (low + high) / 2;
}

#[cfg(kani)]
#[kani::proof]
fn midpoint_overflow() {
    let a: u32 = kani::any();
    let b: u32 = kani::any();
    find_midpoint(a, b);
}
