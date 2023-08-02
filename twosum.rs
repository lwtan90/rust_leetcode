// https://leetcode.com/problems/two-sum/description/ 

/* Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, and you may not use the same element twice.
You can return the answer in any order. */

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        for index in 0..nums.len(){
            for index2 in (index+1)..nums.len(){
                if nums[index] + nums[index2] == target {
                    return vec![index as i32, index2 as i32];
                }
            }
        }

        return vec![0;0];
}
