use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new(); 

    for (index, &value) in nums.iter().enumerate() {
        let complement = target - value;
        if let Some(&comp_index) = map.get(&complement) {
            return vec![comp_index as i32, index as i32];
        }
        map.insert(value, index);
    }

    return vec![];
}

// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
//     for (index_1, value_1) in nums.iter().enumerate() {
//         for (index_2, value_2) in nums.iter().enumerate() {
//             if index_1 != index_2 && value_1 + value_2 == target {
//                 return vec![index_1 as i32, index_2 as i32];
//             }
//         }
//     }
//     vec![]
// }

// pub fn two_sum(mut nums: Vec<i32>, target: i32) -> Vec<i32> {
//     // let mut sorted_num: Vec<i32> = nums.clone();
//     nums.sort();

//     let mut start: usize = 0;
//     let mut end: usize = nums.len() - 1;

//     while start < end {
//         let start_value: i32 = nums[start];
//         let end_value: i32 = nums[end];

//         let value_sum: i32 = start_value + end_value;
//         if value_sum == target {
//             return vec![start as i32, end as i32];
//         } else if value_sum < target {
//             start += 1;
//         } else {
//             end -= 1;
//         }
//     }

//     vec![]
// }
