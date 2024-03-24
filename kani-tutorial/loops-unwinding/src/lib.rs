#[allow(dead_code)]
fn initialize_prefix(length: usize, buffer: &mut [u8]) {
    if length > buffer.len() {
        return;
    }

    for i in 0..=length {
        buffer[i] = 0;
    }
}

#[cfg(kani)]
#[kani::proof]
#[kani::unwind(11)]
fn check_initialize_prefix() {
    const LIMIT: usize = 10;
    let mut buffer: [u8; LIMIT] = [1; LIMIT];

    let length = kani::any();
    kani::assume(length <= LIMIT);

    initialize_prefix(length, &mut buffer);
}

// kani::unwind(1)
// Failed Checks: unwinding assertion loop 0
//  File: "kani-tutorial/loops-unwinding/src/lib.rs", line 7, in initialize_prefix

// kani::unwind(11)
// Failed Checks: index out of bounds: the length is less than or equal to the given index
//  File: "kani-tutorial/loops-unwinding/src/lib.rs", line 8, in initialize_prefix
