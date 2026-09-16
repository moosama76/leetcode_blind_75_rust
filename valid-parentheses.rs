impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];
        for current in s.chars() {
            match current {
                '(' => {stack.push(current);},
                '[' => {stack.push(current);},
                '{' => {stack.push(current);},

                ')' => {if stack.last() == Some(&'(') {stack.pop();} else {stack.push(current);}},
                ']' => {if stack.last() == Some(&'[') {stack.pop();} else {stack.push(current);}},
                '}' => {if stack.last() == Some(&'{') {stack.pop();} else {stack.push(current);}},

                _ => {unreachable!("not a valid input!");},
            }
        }
        stack.is_empty()
    }
}
