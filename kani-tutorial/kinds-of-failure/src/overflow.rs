#[allow(dead_code)]
fn simple_addition(a: u32, b: u32) -> u32 {
    return a + b;
}
// /fix Failed Checks: attempt to add with overflow
// fn simple_addition(a: u32, b: u32) -> Option<u32> {
//     return a.checked_add(b);
// }

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn doesnt_crash(a: u32, b: u32) {
            simple_addition(a, b);
        }
    }
}

#[cfg(kani)]
#[kani::proof]
fn add_overflow() {
    let a: u32 = kani::any();
    let b: u32 = kani::any();
    simple_addition(a, b);
}
