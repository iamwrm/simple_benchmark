// This test is for cache
use simple_benchmark::array_loop;


fn main() {
    array_loop::<1>();
    array_loop::<2>();
    array_loop::<4>();
    array_loop::<8>();
    array_loop::<16>();
    array_loop::<20>();
    array_loop::<21>();
    array_loop::<22>();
    array_loop::<23>();
    array_loop::<25>();
    array_loop::<30>();
    array_loop::<32>();
    array_loop::<64>();
    array_loop::<128>();
    array_loop::<256>();
    array_loop::<512>();
    array_loop::<1024>();
    array_loop::<2048>();
    array_loop::<4096>();
    array_loop::<8192>();
}
