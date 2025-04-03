pub fn delete_and_backspace(s: &mut String) {
    let original: Vec<char> = s.chars().collect();
    let mut result = String::new();
    let mut i = 0;
    while i < original.len() {
        match original[i] {
            '-' => {
                result.pop();
                i += 1;
            }
            '+' => {
                i += 2;
            }
            ch => {
                result.push(ch);
                i += 1;
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
        }
        else if let Some(pos) = eq.find('-') {
            let (left, right) = eq.split_at(pos);
            let right = &right[1..];
            let lhs: i32 = left.trim().parse().unwrap_or(0);
            let rhs: i32 = right.trim().parse().unwrap_or(0);
            *eq = (lhs - rhs).to_string();
        }
    }
}