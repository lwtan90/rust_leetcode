link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock/description/ 
  
/*
Intuition
Buying low and selling high.

Approach
Find the lowest point to date (buy) and sell at the next maximum (high).
Each time you sell, you record the profit. If the next iteration the profit is higher, you replace that with the new profit.

Complexity
Time complexity:
O(n) 4ms beats 99.8% of the solution

Space complexity:
O(1) but still uses 3MB of memory
*/

pub fn max_profit(prices: Vec<i32>) -> i32 {
        // To solve this:
        // 1. find the lowest point
        // 2. find the largest point after the lowest

        // for loop to find the lowest
        let mut min = 100000;
        let mut max = 0;
        let mut min_set = false;
        let mut max_diff = 0;

        for i in 0..prices.len() {
            if prices[i] > max {
                max = prices[i];
            }
            if prices[i] < min {

                if min_set {
                    if max - min > max_diff {
                        max_diff = max - min;
                    }
                }

                min_set = true;
                min = prices[i];
                max = prices[i];
                //println!("{} {} {} {} {} {}",i, min_pos, max_pos, min,max,max_diff);
            }
        }

       if min_set {
           if max - min > max_diff {
                max_diff = max - min;
            }
       }

        //println!("{} {} {} {} {}", min_pos, max_pos, min,max,max_diff);


        return max_diff;
    }
