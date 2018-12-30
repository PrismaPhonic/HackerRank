function minimumAbsoluteDifference(arr) {
    let sorted = arr.slice().sort((a, b) => a - b);
    let min = Math.abs(sorted[0] - sorted[1]);
    for (let i = 1; i < sorted.length - 1; i++) {
        let diff = Math.abs(sorted[i] - sorted[i + 1]);
        if (diff < min) min = diff;
    }
    return min;
}
