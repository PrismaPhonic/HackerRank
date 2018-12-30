# Minimum Absolute Difference

## JS Solution

Here's my JS solution:

```javascript
function minimumAbsoluteDifference(arr) {
    let sorted = arr.slice().sort((a, b) => a - b);
    let min = Math.abs(sorted[0] - sorted[1]);
    for (let i = 1; i < sorted.length - 1; i++) {
        let diff = Math.abs(sorted[i] - sorted[i + 1]);
        if (diff < min) min = diff;
    }
    return min;
}
```

This is the most efficient solution I can come up with.  We get O(n*log(n)) for
the sorting and then O(n) for the iteration through the loop which is better for
very large inputs than quadratic solution. If the array is sorted than our
minimum difference between pairs will always be found to be the difference of
some **adjacent** pair in the sorted array. 
