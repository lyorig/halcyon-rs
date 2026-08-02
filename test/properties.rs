
use halcyon::properties::Properties;
use rustest::test;

static MARKER: u64 = 42;

/// Every property is visited exactly once, with its name.
#[test]
fn properties_enumerate_keys() {
    let mut props = Properties::new().unwrap();
    props.set_number(c"one", 1).unwrap();
    props.set_number(c"two", 2).unwrap();
    props.set_number(c"three", 3).unwrap();

    let mut visited = Vec::new();
    props
        .enumerate(|_props, name| {
            let key = name.to_str().unwrap().to_owned();
            visited.push(key);
        })
        .unwrap();

    visited.sort();
    assert_eq!(visited, ["one", "three", "two"]);
}

/// The properties ID passed to the callback can be used to read values back.
#[test]
fn properties_enumerate_values() {
    let mut props = Properties::new().unwrap();
    props.set_number(c"answer", 42).unwrap();
    props.set_number(c"negative", -7).unwrap();

    let mut values = Vec::new();
    props
        .enumerate(|props, name| {
            let key = name.to_str().unwrap().to_owned();
            let value = props.number(name, -1);
            values.push((key, value));
        })
        .unwrap();

    values.sort();
    assert_eq!(
        values,
        [("answer".to_owned(), 42), ("negative".to_owned(), -7)]
    );
}

/// All property types survive the round-trip through the callback.
#[test]
fn properties_enumerate_all_types() {
    let mut props = Properties::new().unwrap();
    props.set_number(c"num", 10).unwrap();
    props.set_float(c"flt", 2.5).unwrap();
    props.set_string(c"str", c"hello").unwrap();
    props.set_bool(c"bln", true).unwrap();
    props
        .set_pointer(c"ptr", std::ptr::from_ref(&MARKER).cast_mut().cast())
        .unwrap();

    let mut typed = Vec::new();
    props
        .enumerate(|props, name| {
            let key = name.to_str().unwrap().to_owned();
            match key.as_str() {
                "num" => assert_eq!(props.number(name, 0), 10),
                "flt" => assert_eq!(props.float(name, 0.0), 2.5),
                "str" => assert_eq!(props.string(name, c"").to_str().unwrap(), "hello"),
                "bln" => assert_eq!(props.bool(name, false), true),
                "ptr" => assert_eq!(
                    props.pointer(name, std::ptr::null_mut()),
                    std::ptr::from_ref(&MARKER).cast_mut().cast()
                ),
                other => panic!("unexpected property {other}"),
            }
            typed.push(key);
        })
        .unwrap();

    typed.sort();
    assert_eq!(typed, ["bln", "flt", "num", "ptr", "str"]);
}

/// An empty property group never invokes the callback.
#[test]
fn properties_enumerate_empty() {
    let props = Properties::new().unwrap();

    let mut calls = 0;
    props.enumerate(|_props, _name| calls += 1).unwrap();

    assert_eq!(calls, 0);
}
