use std::ffi::CStr;
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

use halcyon::properties::Properties;
use halcyon::resource::Ref;
use rustest::test;

// The enumeration callback is a bare function pointer without userdata, so
// tests collect results in global state.

static VISITED: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn collect_name(_props: Ref<Properties>, name: *const i8) {
    let key = unsafe { CStr::from_ptr(name) }.to_str().unwrap().to_owned();
    VISITED.lock().unwrap().push(key);
}

/// Every property is visited exactly once, with its name.
#[test]
fn properties_enumerate_keys() {
    VISITED.lock().unwrap().clear();

    let mut props = Properties::new().unwrap();
    props.set_number(c"one".as_ptr(), 1).unwrap();
    props.set_number(c"two".as_ptr(), 2).unwrap();
    props.set_number(c"three".as_ptr(), 3).unwrap();

    props.enumerate(collect_name).unwrap();

    let mut visited = VISITED.lock().unwrap().drain(..).collect::<Vec<_>>();
    visited.sort();
    assert_eq!(visited, ["one", "three", "two"]);
}

static NUMBERS: Mutex<Vec<(String, i64)>> = Mutex::new(Vec::new());

fn read_number(props: Ref<Properties>, name: *const i8) {
    let key = unsafe { CStr::from_ptr(name) }.to_str().unwrap().to_owned();
    let value = props.number(name, -1);
    NUMBERS.lock().unwrap().push((key, value));
}

/// The properties ID passed to the callback can be used to read values back.
#[test]
fn properties_enumerate_values() {
    NUMBERS.lock().unwrap().clear();

    let mut props = Properties::new().unwrap();
    props.set_number(c"answer".as_ptr(), 42).unwrap();
    props.set_number(c"negative".as_ptr(), -7).unwrap();

    props.enumerate(read_number).unwrap();

    let mut values = NUMBERS.lock().unwrap().drain(..).collect::<Vec<_>>();
    values.sort();
    assert_eq!(
        values,
        [("answer".to_owned(), 42), ("negative".to_owned(), -7)]
    );
}

static MARKER: u64 = 42;

static TYPED: Mutex<Vec<String>> = Mutex::new(Vec::new());

fn classify(props: Ref<Properties>, name: *const i8) {
    let key = unsafe { CStr::from_ptr(name) }.to_str().unwrap().to_owned();
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
    TYPED.lock().unwrap().push(key);
}

/// All property types survive the round-trip through the callback.
#[test]
fn properties_enumerate_all_types() {
    TYPED.lock().unwrap().clear();

    let mut props = Properties::new().unwrap();
    props.set_number(c"num".as_ptr(), 10).unwrap();
    props.set_float(c"flt".as_ptr(), 2.5).unwrap();
    props.set_string(c"str".as_ptr(), c"hello").unwrap();
    props.set_bool(c"bln".as_ptr(), true).unwrap();
    props
        .set_pointer(
            c"ptr".as_ptr(),
            std::ptr::from_ref(&MARKER).cast_mut().cast(),
        )
        .unwrap();

    props.enumerate(classify).unwrap();

    let mut typed = TYPED.lock().unwrap().drain(..).collect::<Vec<_>>();
    typed.sort();
    assert_eq!(typed, ["bln", "flt", "num", "ptr", "str"]);
}

static CALLS: AtomicUsize = AtomicUsize::new(0);

fn count(_props: Ref<Properties>, _name: *const i8) {
    CALLS.fetch_add(1, Ordering::Relaxed);
}

/// An empty property group never invokes the callback.
#[test]
fn properties_enumerate_empty() {
    CALLS.store(0, Ordering::Relaxed);

    let props = Properties::new().unwrap();
    props.enumerate(count).unwrap();

    assert_eq!(CALLS.load(Ordering::Relaxed), 0);
}
