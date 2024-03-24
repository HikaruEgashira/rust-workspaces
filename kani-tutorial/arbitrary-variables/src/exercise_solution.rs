use crate::inventory::*;
use std::num::NonZeroU32;
use vector_map::VecMap;

#[cfg(kani)]
mod verification {
    use super::*;

    fn any_inventory(bound: u32) -> Inventory {
        let size: u32 = kani::any();
        kani::assume(size <= bound);

        let mut inner = VecMap::new();

        for _ in 0..size {
            let id: ProductId = kani::any();
            let quantity: NonZeroU32 = kani::any();

            inner.insert(id, quantity);
        }

        Inventory { inner }
    }

    #[kani::proof]
    #[kani::unwind(3)]
    pub fn safe_update_with_any() {
        let mut inventory = any_inventory(0);

        let id: ProductId = kani::any();
        let quantity: NonZeroU32 = kani::any();
        assert!(
            quantity.get() != 0,
            "NonZeroU32 is internally a u32 but it should never be 0."
        );

        inventory.update(id, quantity);
        assert!(inventory.get(&id).unwrap() == quantity);
    }
}
