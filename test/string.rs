use rustest::{Result, test};
use std::ops::Deref;

#[test]
fn string_into_boxed_str() -> Result {
    let s = halcyon::pref_path(c"Foo", c"Bar")?;
    let s_copy = s.to_str().to_owned();

    let s_str = s.into_boxed_str();
    assert_eq!(s_str.deref(), &s_copy);

    Ok(())
}
