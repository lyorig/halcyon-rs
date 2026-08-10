use std::ffi::CString;

use halcyon::properties::Properties;
use rustest::{Result, test};

fn c(s: &str) -> CString {
    let mut vec = Vec::with_capacity(s.len() + 1);
    vec.extend_from_slice(s.as_bytes());
    vec.push(b'\0');

    unsafe { CString::from_vec_with_nul_unchecked(vec) }
}

static MARKER: u64 = 42;

/// Every property is visited exactly once, with its name.
#[test]
fn properties_enumerate_keys() -> Result {
    let props = Properties::new()?;
    props.set_number(c"one", 1)?;
    props.set_number(c"two", 2)?;
    props.set_number(c"three", 3)?;

    let mut visited = Vec::new();
    props.enumerate(|k, _v| {
        let key = k.to_owned();
        visited.push(key);
    })?;

    visited.sort();
    assert_eq!(visited, ["one", "three", "two"]);

    Ok(())
}

/// The properties ID passed to the callback can be used to read values back.
#[test]
fn properties_enumerate_values() -> Result {
    let props = Properties::new()?;
    props.set_number(c"answer", 42)?;
    props.set_number(c"negative", -7)?;

    let mut values = Vec::new();
    props.enumerate(|k, _v| {
        let key = k.to_owned();
        let value = props.number(&c(k), -1);
        values.push((key, value));
    })?;

    values.sort();
    assert_eq!(
        values,
        [("answer".to_owned(), 42), ("negative".to_owned(), -7)]
    );

    Ok(())
}

/// All property types survive the round-trip through the callback.
#[test]
fn properties_enumerate_all_types() -> Result {
    let props = Properties::new()?;
    props.set_number(c"num", 10)?;
    props.set_float(c"flt", 2.5)?;
    props.set_string(c"str", Some(c"hello"))?;
    props.set_bool(c"bln", true)?;
    props.set_pointer(c"ptr", std::ptr::from_ref(&MARKER).cast_mut().cast())?;

    let mut typed = Vec::new();
    props.enumerate(|k, _v| {
        let key = k.to_owned();
        let k = &c(k);
        match key.as_str() {
            "num" => assert_eq!(props.number(k, 0), 10),
            "flt" => assert_eq!(props.float(k, 0.0), 2.5),
            "str" => assert_ne!(props.string(k, None), None),
            "bln" => assert!(props.bool(k, false)),
            "ptr" => assert_eq!(
                props.pointer(k, std::ptr::null_mut()),
                std::ptr::from_ref(&MARKER).cast_mut().cast()
            ),
            other => panic!("unexpected property {other}"),
        }
        typed.push(key);
    })?;

    typed.sort();
    assert_eq!(typed, ["bln", "flt", "num", "ptr", "str"]);

    Ok(())
}

/// An empty property group never invokes the callback.
#[test]
fn properties_enumerate_empty() -> Result {
    let props = Properties::new()?;

    let mut calls = 0;
    props.enumerate(|_props, _name| calls += 1)?;

    assert_eq!(calls, 0);

    Ok(())
}
