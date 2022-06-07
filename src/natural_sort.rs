use std::cmp::Ordering;

pub struct NaturalSort {}

impl NaturalSort {
  /// internal common radix
  const RADIX: u32 = 10;

  /// strcmp with natural
  /// # Arguments
  /// * `a` - first string
  /// * `b` - second string
  /// # Return
  /// * `Ordering` - result of comparison
  /// # Example
  /// ```
  /// use std::cmp::Ordering;
  /// use nae::natural_sort::NaturalSort;
  /// assert_eq!(NaturalSort::strcmp_natural("1", "2"), Ordering::Less);
  /// assert_eq!(NaturalSort::strcmp_natural("2", "2"), Ordering::Equal);
  /// assert_eq!(NaturalSort::strcmp_natural("3", "2"), Ordering::Greater);
  /// ```
  pub fn strcmp_natural(a: &str, b: &str) -> Ordering {
    // println!("a: {} ({}), b: {} ({})", a, a.len(), b, b.len());

    if a == b {
      // return Ordering::Equal;
    }

    let a_len = a.len();
    let b_len = b.len();

    let max_len = std::cmp::max(a.len(), b.len());
    // println!("max_len: {}", max_len);

    let mut ret = Ordering::Equal;

    for i in 0..max_len {
      let ac = if i < a_len {
        a.chars().nth(i).unwrap()
      } else {
        '\0'
      };
      let bc = if i < b_len {
        b.chars().nth(i).unwrap()
      } else {
        '\0'
      };

      // println!("a[{}]: {} (0x{:04X})", i, ac, ac as u32);
      // println!("b[{}]: {} (0x{:04X})", i, bc, bc as u32);

      if ac != bc {
        ret = if ac < bc {
          Ordering::Less
        } else {
          Ordering::Greater
        };

        // println!("a: {}, b: {}", ac.is_digit(Radix), bc.is_digit(Radix));
        if ac.is_digit(NaturalSort::RADIX) && bc.is_digit(NaturalSort::RADIX) {
          let mut anum = "";
          let mut bnum = "";

          for ai in i..a_len - 1 {
            // println!("ai: {} -> {}", ai, a.chars().nth(ai).unwrap());
            if !a.chars().nth(ai).unwrap().is_digit(NaturalSort::RADIX) {
              anum = &a[i..ai];
              break;
            }
          }

          for bi in i..b_len - 1 {
            if !b.chars().nth(bi).unwrap().is_digit(NaturalSort::RADIX) {
              bnum = &b[i..bi];
              break;
            }
          }

          println!("anum: {}", anum);
          println!("bnum: {}", bnum);

          // i += std::cmp::min(anum.len(), bnum.len());
        } else if ret == Ordering::Equal {
          continue;
        } else {
          break;
        }
      }
    }

    // println!(
    //   "ret: {}",
    //   match ret {
    //     Ordering::Less => "Less",
    //     Ordering::Greater => "Greater",
    //     Ordering::Equal => "Equal",
    //   }
    // );
    ret
  }
}

#[cfg(test)]
mod tests {
  use crate::natural_sort::NaturalSort;
  use std::cmp::Ordering;

  #[test]
  fn test_strcmp_natural() {
    assert_eq!(NaturalSort::strcmp_natural("1", "2"), Ordering::Less);
    assert_eq!(NaturalSort::strcmp_natural("2", "2"), Ordering::Equal);
    assert_eq!(NaturalSort::strcmp_natural("3", "2"), Ordering::Greater);
  }

  #[test]
  fn test_strcmp_natural_equal() {
    let a = "a100";
    let b = "a100";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Equal);
  }

  #[test]
  fn test_strcmp_natural_equal_alpha() {
    let a = "aaa";
    let b = "aaa";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Equal);
  }

  #[test]
  fn test_strcmp_natural_greater_1() {
    let a = "a100";
    let b = "a10";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Greater);
  }

  #[test]
  fn test_strcmp_natural_greater_2() {
    let a = "a10b";
    let b = "a10a";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Greater);
  }

  #[test]
  fn test_strcmp_natural_greater_3() {
    let a = "a10a100";
    let b = "a10a10";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Greater);
  }

  #[test]
  fn test_strcmp_natural_less_1() {
    let a = "a10";
    let b = "a100";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Less);
  }

  #[test]
  fn test_strcmp_natural_less_2() {
    let a = "a10a";
    let b = "a10b";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Less);
  }

  #[test]
  fn test_strcmp_natural_less_3() {
    let a = "a10a100";
    let b = "a10b10";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Less);
  }

  #[test]
  fn test_strcmp_natural_greater_alpha_1() {
    let a = "baa";
    let b = "aaa";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Greater);
  }

  #[test]
  fn test_strcmp_natural_greater_alpha_2() {
    let a = "aba";
    let b = "aaa";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Greater);
  }

  #[test]
  fn test_strcmp_natural_greater_alpha_3() {
    let a = "aab";
    let b = "aaa";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Greater);
  }

  #[test]
  fn test_strcmp_natural_less_alpha_1() {
    let a = "aaa";
    let b = "baa";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Less);
  }

  #[test]
  fn test_strcmp_natural_less_alpha_2() {
    let a = "aaa";
    let b = "aba";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Less);
  }

  #[test]
  fn test_strcmp_natural_less_alpha_3() {
    let a = "aaa";
    let b = "aab";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Less);
  }

  #[test]
  fn test_strcmp_natural_greater_number_1() {
    let a = "100b";
    let b = "100a";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Greater);
  }

  #[test]
  fn test_strcmp_natural_less_number_1() {
    let a = "100a";
    let b = "100b";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Less);
  }
}
