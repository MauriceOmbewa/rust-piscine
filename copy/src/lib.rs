pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    let exp_c = (c as f64).exp();
    let ln_c = (c.abs() as f64).ln();
    (c, exp_c, ln_c)
}

pub fn str_function(a: String) -> (String, String) {
    let exp_values: String = a
        .split_whitespace()
        .filter_map(|s| s.parse::<f64>().ok())
        .map(|n| n.exp().to_string())
        .collect::<Vec<String>>()
        .join(" ");
    (a, exp_values)
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32>, Vec<f64>) {
    let ln_values: Vec<f64> = b.iter().map(|&n| (n.abs() as f64).ln()).collect();
    (b, ln_values)
}