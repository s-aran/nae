use std::cmp::Ordering;

const Radix: u32 = 10;

pub struct NaturalSort;

impl NaturalSort {
  pub fn strcmp_natural(a: &str, b: &str) -> Ordering {
    println!("a: {} ({}), b: {} ({})", a, a.len(), b, b.len());

    if a == b {
      // return Ordering::Equal;
    }

    let a_len = a.len();
    let b_len = b.len();

    let max_len = std::cmp::max(a.len(), b.len());
    println!("max_len: {}", max_len);

    let mut ret = Ordering::Equal;

    for i in 0..max_len {
      let a_char = a.chars().nth(i).unwrap();
      dbg!(a_char);
    }

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

      println!("a[{}]: {} (0x{:04X})", i, ac, ac as u32);
      println!("b[{}]: {} (0x{:04X})", i, bc, bc as u32);

      if ac != bc {
        ret = if ac < bc {
          Ordering::Less
        } else {
          Ordering::Greater
        };

        println!("a: {}, b: {}", ac.is_digit(Radix), bc.is_digit(Radix));
        if ac.is_digit(Radix) && bc.is_digit(Radix) {
          let mut anum = "";
          let mut bnum = "";

          for ai in i..(max_len) {
            println!("ai: {} -> {}", ai, a.chars().nth(ai).unwrap());
            if !a.chars().nth(ai).unwrap().is_digit(Radix) {
              anum = &a[i..ai];
              break;
            }
          }

          for bi in i..(max_len) {
            if !b.chars().nth(bi).unwrap().is_digit(Radix) {
              bnum = &b[i..bi];
              break;
            }
          }

          println!("anum: {}", anum);
          println!("bnum: {}", bnum);
        } else {
          break;
        }
      }
    }

    println!("ret: {}", match ret {
      Ordering::Less => "Less",
      Ordering::Greater => "Greater",
      Ordering::Equal => "Equal",
    });
    ret
  }
}

#[cfg(test)]
mod tests {
  use crate::natural_sort::NaturalSort;
  use std::cmp::Ordering;

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
  fn test_strcmp_natural_greater() {
    let a = "a100";
    let b = "a10";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == Ordering::Greater);
  }

  #[test]
  fn test_strcmp_natural_less() {
    let a = "a10";
    let b = "a100";

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
}
