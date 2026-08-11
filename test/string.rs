use rustest::{Result, test};

#[test]
fn string_into_boxed_str() -> Result {
    let s = halcyon::pref_path(c"Foo", c"Bar")?;
    let s_copy = s.to_str().to_owned();

    let s_str = s.into_boxed_str();
    assert_eq!(&*s_str, &s_copy);

    Ok(())
}
