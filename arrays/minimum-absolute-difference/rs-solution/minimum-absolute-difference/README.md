# Minimum Absolute Difference

## Rust Solution

It seems that HackerRank didn't even try to setup this problem to be solvable in
Rust. Shame! Regardless I've solved it and ran their tests myself and everything
succeeds in Rust.  Here's my working solution:

```Rust
fn minimum_absolute_difference(arr: &mut Vec<i32>) -> i32 {
    arr.sort_unstable();
    let mut min = (&arr[0] - &arr[1]).abs();
    for i in 1..arr.len()-1 {
        let diff = (&arr[i] - &arr[i+1]).abs();
        if diff < min { min = diff };
    }
    min
}
```

For absolute efficiency I'm relying on a few things - one is that I'm passing in
a mutable reference to the vector and then we call `sort_unstable` because w
don't care about maintaining the order of elements in the vector, we just want
the fastest sort possible (and to mutate in place for performance reasons as
well).  

This is the most efficient solution I can come up with.  We get O(n*log(n)) for
the sorting and then O(n) for the iteration through the loop which is better for
very large inputs than quadratic solution. If the array is sorted than our
minimum difference between pairs will always be found to be the difference of
some **adjacent** pair in the sorted array.
