#[allow(dead_code)]
fn get_wrapped(i: usize, a: &[u32]) -> u32 {
    if a.len() == 0 {
        return 0;
    }
    // return a[i % a.len() + 1];
    // return unsafe { *a.get_unchecked(i % a.len()) };

    // /fix Switch back to the normal/safe indexing operation
    return a[i % a.len()];
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn doesnt_crash(i: usize, a: Vec<u32>) {
            get_wrapped(i, &a);
        }
    }
}

#[cfg(kani)]
#[kani::proof]
fn bound_check() {
    let size: usize = kani::any();
    kani::assume(size < 4096);
    let index: usize = kani::any();
    let array: Vec<u32> = vec![0; size];
    get_wrapped(index, &array);
}

// Failed Checks: index out of bounds: the length is less than or equal to the given index
