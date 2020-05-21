pub fn is_monotonic(a: Vec<i32>) -> bool {
    let mut is_increase = 0;
    let mut is_decrease = 0;
    //isIncrease
    for i in 0..a.len() - 1 {
        if a[i] > a[i + 1] {
            is_increase = is_increase + 0 ;
        } else {
            is_increase = is_increase + 1;
        }
    }
    //isDecrease
    for i in 0..a.len() - 1 {
        if a[i] < a[i + 1] {
            is_decrease = is_decrease + 0 ;
        } else {
            is_decrease = is_decrease + 1;
        }
    }

    if is_decrease == a.len() - 1 {
        return true
    } else if is_increase == a.len() - 1 {
        return true
    } else {
        return false
    }
}

fn main() {
let monotonic = vec![1,1,1];
println!("{}", is_monotonic(monotonic));
}