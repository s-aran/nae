pub struct FullWidth {}

impl FullWidth {
  ///
  /// Convert half-width numeric string to full-width.
  ///
  /// # Examples
  ///
  /// ```
  /// use nae::fullwidth::FullWidth;
  ///
  /// assert_eq!(Fullwidth::halfwidth_to_fullwidth_number("123"), "１２３");
  /// ```
  ///
  pub fn halfwidth_to_fullwidth_number(s: &str) -> String {
    let mut ret = String::new();
    for c in s.chars() {
      if c == '0' {
        ret.push('０');
      } else if c == '1' {
        ret.push('１');
      } else if c == '2' {
        ret.push('２');
      } else if c == '3' {
        ret.push('３');
      } else if c == '4' {
        ret.push('４');
      } else if c == '5' {
        ret.push('５');
      } else if c == '6' {
        ret.push('６');
      } else if c == '7' {
        ret.push('７');
      } else if c == '8' {
        ret.push('８');
      } else if c == '9' {
        ret.push('９');
      } else {
        ret.push(c);
      }
    }
    ret
  }

  ///
  /// Convert full-width numeric string to half-width.
  ///
  /// # Examples
  ///
  /// ```
  /// use nae::fullwidth::Fullwidth;
  ///
  /// assert_eq!(Fullwidth::fullwidth_to_halfwidth_number("１２３"), "123");
  /// ```
  ///
  pub fn fullwidth_to_halfwidth_number(s: &str) -> String {
    let mut ret = String::new();
    for c in s.chars() {
      if c == '０' {
        ret.push('0');
      } else if c == '１' {
        ret.push('1');
      } else if c == '２' {
        ret.push('2');
      } else if c == '３' {
        ret.push('3');
      } else if c == '４' {
        ret.push('4');
      } else if c == '５' {
        ret.push('5');
      } else if c == '６' {
        ret.push('6');
      } else if c == '７' {
        ret.push('7');
      } else if c == '８' {
        ret.push('8');
      } else if c == '９' {
        ret.push('9');
      } else {
        ret.push(c);
      }
    }
    ret
  }
}

#[cfg(test)]
mod tests {
  use super::FullWidth;

  #[test]
  fn test_half_to_full_number_1() {
    let s = "012345678909876543210";
    let r = FullWidth::halfwidth_to_fullwidth_number(s);
    assert_eq!(
      String::from("０１２３４５６７８９０９８７６５４３２１０"),
      r
    );
  }

  #[test]
  fn test_full_to_half_number_1() {
    let s = "０１２３４５６７８９０９８７６５４３２１０";
    let r = FullWidth::fullwidth_to_halfwidth_number(s);
    assert_eq!(String::from("012345678909876543210"), r);
  }

  #[test]
  fn test_full_to_half_number_2() {
    let s = "ほげ１２３Foo456";
    let r = FullWidth::fullwidth_to_halfwidth_number(s);
    assert_eq!(String::from("ほげ123Foo456"), r);
  }

  #[test]
  fn test_half_to_full_number_2() {
    let s = "ほげ１２３Foo456";
    let r = FullWidth::halfwidth_to_fullwidth_number(s);
    assert_eq!(String::from("ほげ１２３Foo４５６"), r);
  }
}
