use module_056_exercises::{longest_str, ArrayBuffer, CowStr};

// ---------------------------------------------------------------------------
// CowStr
// ---------------------------------------------------------------------------

#[test]
fn cowstr_new_is_empty() {
    let c = CowStr::new();
    assert_eq!(c.as_str(), "");
}

#[test]
fn cowstr_from_str() {
    let c = CowStr::from_str("hello");
    assert_eq!(c.as_str(), "hello");
}

#[test]
fn cowstr_to_mut_materializes() {
    let mut c = CowStr::new();
    assert_eq!(c.as_str(), "");
    {
        let s = c.to_mut("world");
        s.push('!');
    }
    assert_eq!(c.as_str(), "world!");
}

#[test]
fn cowstr_to_mut_no_reclone() {
    let mut c = CowStr::from_str("owned");
    // to_mut should NOT re-clone when already Some — it ignores source
    let s = c.to_mut("ignored");
    assert_eq!(s, "owned");
}

#[test]
fn cowstr_into_string() {
    let c = CowStr::from_str("hello");
    assert_eq!(c.into_string(), "hello".to_string());

    let c = CowStr::new();
    assert_eq!(c.into_string(), String::new());
}

#[test]
fn cowstr_clone_works() {
    let c1 = CowStr::from_str("hello");
    let c2 = c1.clone();
    assert_eq!(c2.as_str(), "hello");
    // cloning a new one should also work
    let c3 = CowStr::new();
    let c4 = c3.clone();
    assert_eq!(c4.as_str(), "");
}

// ---------------------------------------------------------------------------
// ArrayBuffer
// ---------------------------------------------------------------------------

#[test]
fn array_buffer_new_is_empty() {
    let buf: ArrayBuffer<i32, 4> = ArrayBuffer::new();
    assert_eq!(buf.len(), 0);
    assert!(buf.is_empty());
}

#[test]
fn array_buffer_push_and_get() {
    let mut buf: ArrayBuffer<i32, 4> = ArrayBuffer::new();
    assert!(buf.push(10).is_ok());
    assert!(buf.push(20).is_ok());
    assert_eq!(buf.len(), 2);
    assert!(!buf.is_empty());
    assert_eq!(buf.get(0), Some(&10));
    assert_eq!(buf.get(1), Some(&20));
    assert_eq!(buf.get(2), None);
}

#[test]
fn array_buffer_at_capacity() {
    let mut buf: ArrayBuffer<i32, 3> = ArrayBuffer::new();
    assert!(buf.push(1).is_ok());
    assert!(buf.push(2).is_ok());
    assert!(buf.push(3).is_ok());
    assert!(buf.push(4).is_err());
    assert_eq!(buf.len(), 3);
}

#[test]
fn array_buffer_different_sizes() {
    let mut buf: ArrayBuffer<char, 2> = ArrayBuffer::new();
    assert!(buf.push('a').is_ok());
    assert!(buf.push('b').is_ok());
    assert_eq!(buf.get(0), Some(&'a'));
}

// ---------------------------------------------------------------------------
// longest_str
// ---------------------------------------------------------------------------

#[test]
fn longest_str_b_is_longer() {
    let a = "hi";
    let b = CowStr::from_str("hello world");
    let result = longest_str(a, b);
    assert_eq!(result.as_str(), "hello world");
}

#[test]
fn longest_str_a_is_longer() {
    let a = "hello world";
    let b = CowStr::from_str("hi");
    let result = longest_str(a, b);
    assert_eq!(result.as_str(), "hello world");
}

#[test]
fn longest_str_tie_returns_owned() {
    let a = "hello";
    let b = CowStr::from_str("hello");
    let result = longest_str(a, b);
    assert_eq!(result.as_str(), "hello");
}
