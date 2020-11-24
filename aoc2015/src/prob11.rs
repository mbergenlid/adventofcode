use std::str::from_utf8_unchecked;

pub fn solve_part_1() -> Password {
    let mut password = Password::new("hxbxwxba");
    password.next();
    password
}

pub fn solve_part_2() -> Password {
    //hxcaabcc -> hxcaabca
    let mut password = Password::new("hxbxwxba");
    password.next();
    password.inc();
    while !password.has_straight() || !password.has_pairs() || !password.has_no_unsecured_chars() {
        password.inc();
    }
    password
}

pub struct Password {
    value: [u8; 8],
}

impl Password {
    fn new<T: AsRef<[u8]>>(v: T) -> Password {
        let mut value = [0; 8];
        for (i, &c) in v.as_ref().iter().enumerate() {
            value[i] = c;
        }
        Password { value }
    }
}

impl Password {

    pub fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(self.value.as_ref()) }
    }

    pub fn next(&mut self) {
        self.next_straight();
        while !self.has_pairs() || !self.has_no_unsecured_chars() {
            self.next_straight();
        }
    }

    fn inc(&mut self) {
        for i in (0..8).rev() {
            self.value[i] += 1;
            if self.value[i] > ('z' as u8) {
                self.value[i] = 'a' as u8;
            } else {
                break;
            }
        }
    }

    fn next_straight(&mut self) -> &str {
        self.inc();
        if !self.has_straight() {
            if self.value[5] <= ('x' as u8) {
                if ((self.value[6] - ('a' as u8)) as u32 * 26)
                    + ((self.value[7] - ('a' as u8)) as u32)
                    < ((self.value[5] - ('a' as u8)) as u32 + 1) * 26
                        + ((self.value[5] - ('a' as u8)) as u32 + 2)
                {
                    self.value[6] = self.value[5] + 1;
                    self.value[7] = self.value[6] + 1;
                } else {
                    self.value[5] += 1;
                    return self.next_straight();
                }
            } else {
                self.value[5] = 'a' as u8;
                self.value[6] = 'a' as u8;
                self.value[7] = ('a' as u8) - 1;

                for i in (0..5).rev() {
                    self.value[i] += 1;
                    if self.value[i] > ('z' as u8) {
                        self.value[i] = 'a' as u8;
                    } else {
                        break;
                    }
                }
                return self.next_straight();
            }
        }
        unsafe { from_utf8_unchecked(self.value.as_ref()) }
    }

    fn has_straight(&self) -> bool {
        for i in 0..6 {
            if self.value[i + 1] == self.value[i] + 1 && self.value[i + 2] == self.value[i] + 2 {
                return true;
            }
        }
        return false;
    }

    fn has_pairs(&self) -> bool {
        let mut first_pair: Option<usize> = None;
        for i in 0..7 {
            if let Some(prev) = first_pair {
                if self.value[i] == self.value[i+1] && i != prev {
                    return true;
                }
            } else {
                if self.value[i] == self.value[i+1] {
                    first_pair = Some(i+1);
                }
            }
        }
        false
    }

    fn has_no_unsecured_chars(&self) -> bool {
        for &x in self.value.iter() {
            if x == ('i' as u8) || x == ('l' as u8) || x == ('o' as u8) {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod test {
    use crate::prob11::Password;

    #[test]
    fn test_next_straight() {
        assert_eq!(Password::new("hxbxwxba").next_straight(), "hxbxwxyz");
        assert_eq!(Password::new("hxbxaxya").next_straight(), "hxbxaxyz");
        assert_eq!(Password::new("hxbxwyaa").next_straight(), "hxbxxabc");

        assert_eq!(Password::new("hxbxxabc").next_straight(), "hxbxxbcd");

        assert_eq!(Password::new("hxbcaaaa").next_straight(), "hxbcaabc");
        assert_eq!(Password::new("hxbccyaa").next_straight(), "hxbcdaaa");

        assert_eq!(Password::new("abcaaaaa").next_straight(), "abcaaaab");
        assert_eq!(Password::new("hxbxwxya").next_straight(), "hxbxwxyb");

        assert_eq!(Password::new("abcazzzz").next_straight(), "abcbaaaa");
        assert_eq!(Password::new("abczzzzz").next_straight(), "abdaaabc");

        assert_eq!(Password::new("abbzzzzz").next_straight(), "abcaaaaa");
        assert_eq!(Password::new("abbzzzza").next_straight(), "abcaaaaa");

        assert_eq!(Password::new("abbyzzza").next_straight(), "abbzaabc");
        // hxbxwxba -> hxbxwxyz -> hxbxxabc -> hxbxxyza
        // hxbxwxba -> hxbyaabb ->
    }
}
