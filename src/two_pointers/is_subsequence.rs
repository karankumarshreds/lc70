fn is_subsequence(s: String, t: String) -> bool {
    let mut s = s.chars();
    let mut curr = s.next();
    for c in t.chars() {
        if Some(c) == curr {
            curr = s.next();
        }
    }
    curr.is_none()
    // Generic solution
    //        let s = s.chars().collect::<Vec<char>>();
    //        let t = t.chars().collect::<Vec<char>>();
    //        let mut i = 0;
    //        let mut j = 0;
    //        while j < t.len() && i < s.len() {
    //            if t[j] == s[i] {
    //                i += 1;
    //            }
    //            j += 1;
    //        }
    //        return s.len() == i
}

pub fn run() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");
    let res = is_subsequence(s, t);
    assert_eq!(res, true, "is_subsequence.rs");
    let s = String::from("axc");
    let t = String::from("ahbgdc");
    let res = is_subsequence(s, t);
    assert_eq!(res, false, "is_subsequence.rs");
    let s = String::from("a");
    let t = String::from("b");
    let res = is_subsequence(s, t);
    assert_eq!(res, false, "is_subsequence.rs for data a, b");
}
