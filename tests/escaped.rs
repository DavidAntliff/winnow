use nom::bytes::escaped;
use nom::bytes::one_of;
use nom::character::digit1;
use nom::{error::ErrorKind, Err, IResult};

fn esc(s: &str) -> IResult<&str, &str, (&str, ErrorKind)> {
  escaped(digit1, '\\', one_of("\"n\\"))(s)
}

#[cfg(feature = "alloc")]
fn esc_trans(s: &str) -> IResult<&str, String, (&str, ErrorKind)> {
  use nom::bytes::{escaped_transform, tag};
  escaped_transform(digit1, '\\', tag("n"))(s)
}

#[test]
fn test_escaped() {
  assert_eq!(esc("abcd"), Err(Err::Error(("abcd", ErrorKind::Escaped))));
}

#[test]
#[cfg(feature = "alloc")]
fn test_escaped_transform() {
  assert_eq!(
    esc_trans("abcd"),
    Err(Err::Error(("abcd", ErrorKind::EscapedTransform)))
  );
}
