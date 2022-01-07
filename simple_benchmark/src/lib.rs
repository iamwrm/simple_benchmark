pub fn array_loop<const ARRAY_LEN: usize>() {
    // const ARRAY_LEN: usize = 10000;
    let mut array = [0u64; ARRAY_LEN];

    let start = std::time::Instant::now();

    for _ in 0..(1000000 / ARRAY_LEN) {
        for i in array.iter_mut() {
            let mut step = 1;
            loop {
                if step < u64::MAX / 2 {
                    step *= 2;
                }
                if u64::MAX - step > *i {
                    *i += step;
                } else {
                    break;
                }
            }
        }
        for i in array.iter_mut() {
            *i = 0;
        }
    }
    println!(
        "Elapsed time: {} ms | {} {}",
        start.elapsed().as_millis(),
        ARRAY_LEN,
        ARRAY_LEN * 64,
    );
}
