#[macro_export]
macro_rules! nested {
    ($start:ident $(. $attrs:ident)+ -> $op:ident $(-> $tail:ident)*) => {{
        let cur = unsafe { $start$(. $attrs)*.$op.as_ref() };
        if let Some(cur) = cur {
            nested!(cur $(-> $tail)*)
        } else {
            None
        }
    }};

    ($start:ident -> $op:ident) => {{
        unsafe { $start.$op.as_ref() }
    }};

    ($start:ident -> $op:ident $(-> $tail:ident)+ ) => {{
        let cur = unsafe { $start.$op.as_ref() };
        if let Some(cur) = cur {
            nested!(cur $(-> $tail)*)
        } else {
            None
        }
    }};
}

#[macro_export]
macro_rules! match_or_continue {
    ($e:expr) => {
        match $e {
            Some(opt) => opt,
            None => continue,
        }
    };
}
