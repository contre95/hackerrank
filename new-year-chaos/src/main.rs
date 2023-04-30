fn minimum_bribes(q: &[i32]) {
    let mut count: usize = 0;
    let mut pivot: i32 = 0;
    while count < q.len() {
        let a = q[count] - (count as i32) - 1;
        // println!("{}", a);
        if a > 2 {
            println!("Too chaotic");
            return;
        } else if a > 0 {
            pivot += a;
        }
        count += 1;
    }
    println!("{}", pivot);
}

fn main() {
    let q = [1, 2, 5, 3, 7, 8, 6, 4];
    minimum_bribes(&q);
    // let q = [2, 1, 5, 3, 4];
    // minimum_bribes(&q);
    // let q = [2, 5, 1, 3, 4];
    // minimum_bribes(&q);
}

