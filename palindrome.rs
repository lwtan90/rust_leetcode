/*Given an integer x, return true if x is a 
palindrome, and false otherwise.*/

mpl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let mut s = x.to_string();
        let mut vec = Vec::new();
        for c in s.chars(){
            vec.push(c);
        }
        let mut left = 0;
        let mut right = vec.len()-1;
        while left<right {
            if vec[left]!=vec[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        return true;
    }
}
