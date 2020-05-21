pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut ans = vec![];
        let mut max = 0;
        // let mut max = candies.iter().max();
        for number in candies.iter() {
            if max < *number {
                max = *number;
            }
        }
        for number in candies.iter() {
            if *number + extra_candies >= max {
                ans.push(true);
            } else {
                ans.push(false);
            } 
        }
        return ans.to_vec();
    }

fn main() {
    let candies = vec![2,3,5,1,3];
    let extra_candies = 3;
    println!("{:?}", kids_with_candies(candies, extra_candies));
}