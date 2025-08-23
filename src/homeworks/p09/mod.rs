pub fn rotate(s: String, n: isize) -> String {
    if s.is_empty() {
        return s;
    }

    let len = s.len() as isize;
    let n = n % len;
    
    if n == 0 {
        return s;
    }

    let split_pos = if n > 0 {
        (len - n) as usize
    } else {
        (-n) as usize
    };

    let (left, right) = s.split_at(split_pos);
    format!("{}{}", right, left)
}

#[test]
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0,  "abcdefgh"),
        (8,  "abcdefgh"),
        (-8, "abcdefgh"),
        (1,  "habcdefg"),
        (2,  "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10,"cdefghab"),
    ];

    shifts
        .iter()
        .for_each(|(n, exp)|
            assert_eq!(
                rotate(s.to_string(), *n), 
                exp.to_string()
            )
        );
}
