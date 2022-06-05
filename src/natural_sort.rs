const Radix: u32 = 10;

#[derive(Hash, Eq, PartialEq, Debug)]
pub enum CompareResult {
  Equal = 0,
  Greater = 1,
  Less = -1,
}

pub struct NaturalSort;

impl NaturalSort {
  pub fn strcmp_natural(a: &str, b: &str) -> CompareResult {
    println!("a: {} ({}), b: {} ({})", a, a.len(), b, b.len());

    if a == b {
      return CompareResult::Equal;
    }

    let a_len = a.len();
    let b_len = b.len();

    let max_len = std::cmp::max(a.len(), b.len()) - 1;
    println!("max_len: {}", max_len);

    let mut ret = CompareResult::Equal;

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

      println!("a[{}]: {}", i, ac);
      println!("b[{}]: {}", i, bc);

      if ac != bc {
        ret = if ac > bc {
          CompareResult::Less
        } else {
          CompareResult::Greater
        };
      };

      if ac.is_digit(Radix) && bc.is_digit(Radix) {
        let mut anum = "";
        let mut bnum = "";

        for ai in i..(max_len ) {
          println!("ai: {} -> {}", ai, a.chars().nth(ai).unwrap());
          if !a.chars().nth(ai).unwrap().is_digit(Radix) {
            anum = &a[i..ai];
            break;
          }
        }

        for bi in i..(max_len ) {
          if !b.chars().nth(bi).unwrap().is_digit(Radix) {
            bnum = &b[i..bi];
            break;
          }
        }

        println!("anum: {}", anum);
        println!("bnum: {}", bnum);
      }
    }

    ret
  }
}

#[cfg(test)]
mod tests {
  use crate::natural_sort::CompareResult;
  use crate::natural_sort::NaturalSort;

  #[test]
  fn test_strcmp_natural_equal() {
    let a = "a100";
    let b = "a100";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == CompareResult::Equal);
  }

  #[test]
  fn test_strcmp_natural_greater() {
    let a = "a100";
    let b = "a10";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == CompareResult::Greater);
  }

  #[test]
  fn test_strcmp_natural_less() {
    let a = "a10";
    let b = "a100";

    let result = NaturalSort::strcmp_natural(a, b);
    assert!(result == CompareResult::Less);
  }

  #[test]
  fn test_strcmp_natural_greater_alpha() {
    let a = "aba";
    let b = "aaa";

    let result = a.cmp(b);
    assert!(result == std::cmp::Ordering::Greater);
  }

  #[test]
  fn test_strcmp_natural_less_alpha() {
    let a = "aaa";
    let b = "aba";

    let result = a.cmp(b);
    assert!(result == std::cmp::Ordering::Less);
  }
}
