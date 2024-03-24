#[allow(dead_code)]
fn estimate_size(x: u32) -> u32 {
    if x < 256 {
        if x < 128 {
            return 1;
        } else {
            return 3;
        }
    } else if x < 1024 {
        if x > 1022 {
            panic!("Oh no, a failing corner case!");
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
        fn doesnt_crash(x: u32) {
            estimate_size(x);
        }
    }
}

#[cfg(kani)]
#[kani::proof]
fn check_estimate_size() {
    let x: u32 = kani::any();
    estimate_size(x);
}
#[test]
fn kani_concrete_playback_check_estimate_size_14615086421508420155() {
    let concrete_vals: Vec<Vec<u8>> = vec![
        // 1023
        vec![255, 3, 0, 0],
    ];
    kani::concrete_playback_run(concrete_vals, check_estimate_size);
}
