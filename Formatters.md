```
#[test]
fn test() {
    assert_eq!(add_comma_separators(0), "0");
    assert_eq!(add_comma_separators(1), "1");
    assert_eq!(add_comma_separators(999), "999");
    assert_eq!(add_comma_separators(1000), "1,000");
    assert_eq!(add_comma_separators(999999), "999,999");
    assert_eq!(add_comma_separators(1000000), "1,000,000");
    assert_eq!(add_comma_separators(999999999), "999,999,999");
    assert_eq!(add_comma_separators(1000000000), "1,000,000,000");
    assert_eq!(add_comma_separators(999999999999), "999,999,999,999");
    assert_eq!(add_comma_separators(1000000000000), "1,000,000,000,000");
}
```
