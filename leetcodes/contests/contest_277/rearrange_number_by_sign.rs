fn main() {
    let v = vec![3,1,-2,-5,2,-4];
    rearrange_array(v);
}

pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {

    if nums.len() == 0 {
        return Vec::new();
    }
    let mut pos_pointer = 0;
    let mut neg_pointer = 0;
    let len = nums.len();
    let mut result: Vec<i32> = Vec::new();
    let mut cur = 0;

    println!("{} len", len);
    while cur < len {
        while nums[pos_pointer] < 0 && pos_pointer < len {
            pos_pointer += 1;
        }
        if pos_pointer < len {
            result.push(nums[pos_pointer]);
            cur += 1;
            pos_pointer +=1;
        }

        while nums[neg_pointer] >= 0 && neg_pointer < len {
            neg_pointer += 1;
        }
        if neg_pointer < len {
            result.push(nums[neg_pointer]);
            cur += 1;
            neg_pointer +=1;
        }
    }
    return result;

}