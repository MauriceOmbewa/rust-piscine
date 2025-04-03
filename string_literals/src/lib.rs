pub fn is_empty(v: &str) -> bool {
    if v.len() == 0{
        return true;
    }
    false
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    // let tuple: (String, String) = (v[..index].to_string(), v[index..].to_string());
    // tuple
    (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
    match v.find(pat){
        Some(index) => index as usize,
        None => usize::MAX,
    }
}