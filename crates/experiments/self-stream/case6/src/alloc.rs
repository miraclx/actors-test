use std::{
    alloc::{GlobalAlloc, Layout, System},
    fs::File,
    io::Write,
    mem::ManuallyDrop,
    os::fd::FromRawFd,
};

pub struct MyAllocator;

fn digits_iterator(mut n: u64) -> impl Iterator<Item = u64> {
    let log2 = 0_u64.count_zeros() - n.leading_zeros();
    let log10 = ((log2 + 1) * 1233) >> 12;
    let mut splitter = 10_u64.pow(log10);

    if n < splitter {
        splitter /= 10;
    }

    std::iter::from_fn(move || {
        if splitter > 0 {
            let digit = n / splitter;
            n %= splitter;
            splitter /= 10;
            Some(digit)
        } else {
            None
        }
    })
}

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut msg = *b"allocating 00000000000000 bytes\n";

        for (i, digit) in digits_iterator(layout.size() as u64).enumerate() {
            let digit = digit as u8 + b'0';
            msg[msg.len() - (8 + i)] = digit;
        }

        let mut stdout = ManuallyDrop::new(File::from_raw_fd(1));
        let _ignored = stdout.write(&msg);

        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut msg = *b"deallocating 00000000000000 bytes\n";

        for (i, digit) in digits_iterator(layout.size() as u64).enumerate() {
            let digit = digit as u8 + b'0';
            msg[msg.len() - (8 + i)] = digit;
        }

        let mut stdout = ManuallyDrop::new(File::from_raw_fd(1));
        let _ignored = stdout.write(&msg);

        System.dealloc(ptr, layout)
    }
}
