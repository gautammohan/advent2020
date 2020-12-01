pub fn d1p1(nums : &Vec<i32>) -> i32 {
    let n = nums.len();
    for i in 0..n {
        for j in i..n {
            if nums[i] + nums[j] == 2020 {
                return nums[i] * nums[j]
            }
        }
    }
    panic!("not found");
}

pub fn d1p2(nums : &Vec<i32>) -> i32 {
    let n = nums.len();
    for i in 0..n {
        for j in i..n {
            let x = nums[i];
            let y = nums[j];
            if let Some(&z) = nums.iter().find(|&&n| n == 2020-x-y) {
                if z != x && z != y {
                    return x * y * z
                }
            }
        }
    }
    panic!("not found");
}
