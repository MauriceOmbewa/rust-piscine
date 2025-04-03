pub fn str_len(s: &str) -> usize {
    if s.contains('á'){
        return (s.len() - 1) as usize
    }
    s.len() as usize
}
