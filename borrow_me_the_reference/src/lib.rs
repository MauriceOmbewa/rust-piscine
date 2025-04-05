pub fn delete_and_backspace(s: &mut String) {
    let mut result = String::new();
    let mut skip = 0;
    for ch in s.chars() {
        match ch {
            '-' => { 
                result.pop();
            }
            '+' => {
                skip += 1;
            }
            _ => {
                if skip > 0 {
                    skip -= 1;
                } else {
                    result.push(ch);
                }
            }
        }
    }
    *s = result;
}

pub fn do_operations(v: &mut [String]) {
    for eq in v.iter_mut() {
        if let Some(pos) = eq.find('+') {
            let (left, right) = eq.split_at(pos);
            let right = &right[1..];
            let lhs: i32 = left.trim().parse().unwrap_or(0);
            let rhs: i32 = right.trim().parse().unwrap_or(0);
            *eq = (lhs + rhs).to_string();
        } else if let Some(pos) = eq.find('-') {
            let (left, right) = eq.split_at(pos);
            let right = &right[1..];
            let lhs: i32 = left.trim().parse().unwrap_or(0);
            let rhs: i32 = right.trim().parse().unwrap_or(0);
            *eq = (lhs - rhs).to_string();
        }
    }
}