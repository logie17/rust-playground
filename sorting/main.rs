fn main() {
    let mut numbers = vec![30, 25, 10, 40, 99, 80, 33];
    qsort_it(&mut numbers, 0, 6);

    println!("Sorted values are {:?}", numbers);

}

fn qsort_it(a: &mut Vec<i32>, left: usize, right: usize) {
    let i: usize;
    let mut last: usize = left;

    if left >= right {
        return;
    }

    swap_it(a, left, (left + right)/2);

    i = left + 1;

    for x in i..right+1 {
        if a[x] < a[left] {
            last += 1;
            swap_it(a, last, x);
        }
    }

    if last < 1 {
        return
    }
    swap_it(a, left, last);
    qsort_it(a, left, last-1);
    qsort_it(a, last + 1, right);

}

fn swap_it(a: &mut Vec<i32>, i: usize, j: usize) {
    let tmp: i32 = a[i];
    a[i] = a[j];
    a[j] = tmp;
}
