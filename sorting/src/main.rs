fn main() {
    let mut numbers = vec![30, 25, 10, 40, 99, 80, 33];
    qsort(&mut numbers);
    println!("Sorted values are {:?}", numbers);
}

fn qsort(a: &mut Vec<i32>) {
    let length = a.len() - 1;
    qsort_it(a, 0, length);
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut numbers = vec![1, 5, 3, 4, 10, 20, 15];
        qsort(&mut numbers);
        assert_eq!(numbers, vec![1, 3, 4, 5, 10, 15, 20]);
    }
}
