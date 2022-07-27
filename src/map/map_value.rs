///ViperusValue encaspule data values of type String,i32 and bool
///
///implements bidirectional conversion to respective  values via Into<T> and From<T>
/// # Example
/// ```
/// use viperus::ViperusValue;
/// let x:i32=ViperusValue::I32(42).into();
/// ```
///
#[derive(Debug, PartialEq, Clone)]
pub enum ViperusValue {
    Empty,
    Str(String),
    I32(i32),
    BOOL(bool),
}

impl From<ViperusValue> for bool {
    fn from(src: ViperusValue) -> bool {
        match src {
            ViperusValue::BOOL(i) => i,
            ViperusValue::Str(s) => s.parse().expect("not a bool"),
            _ => panic!("not a bool {:?}", src),
        }
    }
}

impl From<&ViperusValue> for bool {
    fn from(src: &ViperusValue) -> bool {
        match src {
            ViperusValue::BOOL(i) => *i,
            ViperusValue::Str(s) => s.parse().expect("not a bool"),
            _ => panic!("not a bool {:?}", src),
        }
    }
}

impl From<bool> for ViperusValue {
    fn from(src: bool) -> ViperusValue {
        ViperusValue::BOOL(src)
    }
}

impl From<i32> for ViperusValue {
    fn from(src: i32) -> ViperusValue {
        ViperusValue::I32(src)
    }
}

impl From<&ViperusValue> for i32 {
    fn from(src: &ViperusValue) -> i32 {
        match src {
            ViperusValue::I32(i) => *i,
            ViperusValue::Str(s) => s.parse().expect("not an i32"),
            _ => panic!("not an i32"),
        }
    }
}

impl From<ViperusValue> for i32 {
    fn from(src: ViperusValue) -> i32 {
        match src {
            ViperusValue::I32(i) => i,
            ViperusValue::Str(s) => s.parse().expect("not an i32"),
            _ => panic!("not an i32"),
        }
    }
}

impl From<String> for ViperusValue {
    fn from(src: String) -> ViperusValue {
        ViperusValue::Str(src)
    }
}

impl<'a> From<&'a String> for ViperusValue {
    fn from(src: &'a String) -> ViperusValue {
        ViperusValue::Str(src.clone())
    }
}

impl From<&str> for ViperusValue {
    fn from(src: &str) -> ViperusValue {
        ViperusValue::Str(src.to_owned())
    }
}

impl<'a> From<&'a ViperusValue> for &'a str {
    fn from(src: &'a ViperusValue) -> &'a str {
        match src {
            ViperusValue::Str(i) => i,
            _ => panic!("not an str"),
        }
    }
}

impl From<&ViperusValue> for String {
    fn from(src: &ViperusValue) -> String {
        match src {
            ViperusValue::Str(i) => i.clone(),
            _ => panic!("not an str"),
        }
    }
}

impl From<ViperusValue> for String {
    fn from(src: ViperusValue) -> String {
        match src {
            ViperusValue::Str(i) => i,
            _ => panic!("not an str"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init() {
        let _ = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[should_panic]
    fn invalid_cast_mv2bool() {
        init();

        let mv = ViperusValue::Empty;
        let _b: bool = mv.into();
    }

    #[test]
    #[should_panic]
    fn invalid_cast_refmv2bool() {
        init();

        let mv = &ViperusValue::Empty;
        let _b: bool = mv.into();
    }

    #[test]
    #[should_panic]
    fn invalid_cast_mv2i32() {
        init();

        let mv = &ViperusValue::Empty;
        let _b: i32 = mv.into();
    }
    #[test]
    fn valid_cast_mv2bool() {
        init();

        let mv = ViperusValue::BOOL(true);
        let b: bool = mv.into();
        assert!(b);
    }

    #[test]
    fn valid_cast_str2mv() {
        init();

        let mv = ViperusValue::from("hello world!");
        match mv {
            ViperusValue::Str(s) => assert_eq!(s, "hello world!"),
            _ => panic!("something very wrong"),
        }

        let refmv = ViperusValue::from(&("hello world!".to_owned()));
        match refmv {
            ViperusValue::Str(s) => assert_eq!(s, "hello world!"),
            _ => panic!("something very wrong"),
        }
    }

    #[test]
    #[should_panic]
    fn invalid_cast_mv2string() {
        init();

        let mv = &ViperusValue::Empty;
        let _b: String = mv.into();
    }
}
