pub fn reverse_bytes(span: &mut [u8]) {
    let len = span.len();
    let (front, back) = span.split_at_mut(len / 2);
    let fwd_bytes_s = front.iter_mut();
    let rev_bytes_s = back.iter_mut().rev();
    for (fwd, bck) in std::iter::zip(fwd_bytes_s, rev_bytes_s) {
        std::mem::swap(fwd, bck);
    }
}

fn chop_last<'a>(body: &'a mut [u8], seps: &[char]) -> &'a mut[u8] {
    let x = std::str::from_utf8(body).unwrap();
    let mut split = 0;
    for (i, c) in x.char_indices().rev() {
        if seps.contains(&'|') {
            split = i;
            break
        } else if seps.contains(&c) {
            split = i + 1;
            break
        }
    }
    let mid = body.get_mut(split..).unwrap();
    mid
}

pub fn reverse_chars(s: String) -> String {
    let seps = vec!['|'];
    reverse(s, &seps)
}

pub fn reverse_words(s: String) -> String {
    let seps = vec![' ', '|'];
    reverse(s, &seps)
}

pub fn reverse_sentences(s: String) -> String {
    let seps = vec!['.', ' ', '|'];
    reverse(s, &seps)
}

pub fn reverse_paragraphs(s: String) -> String {
    let seps = vec!['\n', '.', ' ', '|'];
    reverse(s, &seps)
}

fn reverse_any<'a, 'b>(bytes: &'a mut [u8], seps: &'b Vec<char>, level: usize) -> &'a mut[u8] {
    if seps.len() == level {
        reverse_bytes(bytes);
        return bytes
    }

    let mut split = bytes.len();
    let sep = seps[level];
    while split > 0 {
        let head_and_mid = bytes.get_mut(..split).unwrap();
        let sub_seps = &seps[..level + 1];
        let mid = chop_last(head_and_mid, sub_seps);
        split -= mid.len();

        reverse_any(mid, seps, level + 1);

        if split > 0 && sep != '|' {
            split -= 1; // account for length of separator
        }
    }

    reverse_any(bytes, seps, level + 1);
    bytes
}

fn reverse(s: String, seps: &Vec<char>) -> String {
    let mut bytes = s.into_bytes();
    reverse_any(bytes.as_mut_slice(), seps, 0);
    std::string::String::from_utf8(bytes).unwrap()
}

#[test]
fn it_can_reverse_paragraphs () {
    let s = "
        ave. r??ve l??ve
        no m?? ??gain.
    ".to_string();
    let s = reverse_paragraphs(s);
    assert_eq!("
        no m?? ??gain.
        ave. r??ve l??ve
    ".trim(), s.trim());
}


#[test]
fn it_can_reverse_sentences () {
    let s = "ave. r??ve l??ve".to_string();
    let s = reverse_sentences(s);
    assert_eq!(" r??ve l??ve.ave", s);
}

#[test]
fn it_can_reverse_words () {
    let s = "r??ve l??ve".to_string();
    let s = reverse_words(s);
    assert_eq!("l??ve r??ve", s);
}

#[test]
fn it_can_reverse_chars () {
    let mut s = "r??ve l??ve".to_string();
    let ptr = s.as_mut_ptr();
    let len = s.len();
    let cap = s.capacity();
    let t = reverse_chars(s);
    assert_eq!("ev??l ev??r", t);

    // Double-check that we actually modified the original.
    unsafe {
        let orig_s = String::from_raw_parts(ptr, len, cap);
        assert_eq!("ev??l ev??r", orig_s);
        std::mem::forget(orig_s);
    }
}
