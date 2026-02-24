// The Slice Type
/*
Slices let you reference a contiguous sequence of elements in a collection. A slice is a kind of reference, so it does not have ownership.


*/
pub fn slice_type(){
    let val = String::from("hello this word");
    println!("{}", first_word(&val));
}
fn first_word(s: &String)  -> &str {
    let bytes = s.as_bytes();
    let ss:[usize;4];
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]

}
