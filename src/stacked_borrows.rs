pub fn basic_borrow() -> () {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;

        // ORDER SWAPPED!
        *ref1 += 1;
        *ptr2 += 2;

        println!("{}", data);
    }
}

pub fn complex_borrow() -> () {
    unsafe {
        let mut data = 10;
        let ref1 = &mut data;
        let ptr2 = ref1 as *mut _;
        let ref3 = &mut *ptr2;
        let ptr4 = ref3 as *mut _;

        // Access the first raw pointer first
        *ptr2 += 2;

        // Then access things in "borrow stack" order
        *ptr4 += 4;
        *ref3 += 3;
        *ptr2 += 2;
        *ref1 += 1;

        println!("{}", data);
    }
}

pub fn borrow_arrays() -> () {
    unsafe {
        let mut data = [0; 10];

        let slice1_all = &mut data[..]; // Slice for the entire array
        let ptr2_all = slice1_all.as_mut_ptr(); // Pointer for the entire array

        let ptr3_at_0 = ptr2_all; // Pointer to 0th elem (the same)
        let ptr4_at_1 = ptr2_all.add(1); // Pointer to 1th elem
        let ref5_at_0 = &mut *ptr3_at_0; // Reference to 0th elem
        let ref6_at_1 = &mut *ptr4_at_1; // Reference to 1th elem

        *ref6_at_1 += 6;
        *ref5_at_0 += 5;
        *ptr4_at_1 += 4;
        *ptr3_at_0 += 3;

        // Just for fun, modify all the elements in a loop
        // (Could use any of the raw pointers for this, they share a borrow!)
        for idx in 0..10 {
            *ptr2_all.add(idx) += idx;
        }

        // Safe version of this same code for fun
        for (idx, elem_ref) in slice1_all.iter_mut().enumerate() {
            *elem_ref += idx;
        }

        // Should be [8, 12, 4, 6, 8, 10, 12, 14, 16, 18]
        println!("{:?}", &data[..]);
    }
}

#[cfg(test)]
mod test {
    use super::{basic_borrow, borrow_arrays, complex_borrow};

    #[test]
    fn basics() {
        basic_borrow();
    }

    #[test]
    fn complex() {
        complex_borrow();
    }

    #[test]
    fn arrays() {
        borrow_arrays();
    }
}
