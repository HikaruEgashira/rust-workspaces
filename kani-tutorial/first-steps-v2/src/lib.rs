#[allow(dead_code)]
fn estimate_size(x: u32) -> u32 {
    assert!(x < 4096);

    if x < 256 {
        if x < 128 {
            return 1;
        } else {
            return 3;
        }
    } else if x < 1024 {
        if x > 1022 {
            return 4;
        } else {
            return 5;
        }
    } else {
        if x < 2048 {
            return 7;
        } else {
            return 9;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn it_works() {
        assert_eq!(estimate_size(1024), 7);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(10000))]
        #[test]
        fn doesnt_crash(x in 0..4095u32) {
            estimate_size(x);
        }
    }
}

#[cfg(kani)]
#[kani::proof]
fn verify_success() {
    let x: u32 = kani::any();
    kani::assume(x < 4096);
    let y = estimate_size(x);
    assert!(y < 10);
}

#[cfg(kani)]
#[kani::proof]
fn will_fail() {
    let x: u32 = kani::any();
    let y = estimate_size(x);
}
#[test]
fn kani_concrete_playback_will_fail_14715535830925848214() {
    let concrete_vals: Vec<Vec<u8>> = vec![
        // 4294967295
        vec![255, 255, 255, 255],
    ];
    kani::concrete_playback_run(concrete_vals, will_fail);
}
