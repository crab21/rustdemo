pub fn sort_tuple_ln(tuple_ln: &mut Vec<(i32, i32)>) {
    tuple_ln.sort_by(|a, b| a.0.cmp(&b.0));
}
