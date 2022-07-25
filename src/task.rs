pub trait Task<T> {
  fn execute(&self, target: &mut T) -> bool;
}

pub struct Invoker<'a, T: 'a> {
  tasks: Vec<Box<dyn Task<T> + 'a>>,
  target: &'a mut T,
  current_index: usize,
}

impl<'a, T> Invoker<'a, T> {
  fn new(target: &'a mut T) -> Self {
    Self {
      tasks: vec![],
      target,
      current_index: 0,
    }
  }

  fn get_target(&mut self) -> &mut T {
    &mut self.target
  }

  fn add<U: Task<T> + 'a>(&mut self, task: U) {
    self.tasks.push(Box::new(task));
  }

  fn execute(&mut self) -> bool {
    if self.tasks.len() <= self.current_index {
      // NOP
      return false;
    }

    let c = self.tasks.get(self.current_index).unwrap();
    let t = &mut *self.target;

    if c.execute(t) {
      self.current_index += 1;
      return true;
    }

    false
  }

  fn execute_all(&mut self) -> bool {
    let mut result = true;

    for _ in self.current_index..self.tasks.len() {
      result &= self.execute();
    }

    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::fs;

  struct Target {
    val: bool,
  }

  impl Target {
    fn new() -> Self {
      Self { val: false }
    }

    fn get(&self) -> bool {
      self.val
    }
  }

  struct UpdateTrueTask;
  impl Task<Target> for UpdateTrueTask {
    fn execute(&self, target: &mut Target) -> bool {
      target.val = true;
      true
    }
  }

  struct UpdateFalseTask;
  impl Task<Target> for UpdateFalseTask {
    fn execute(&self, target: &mut Target) -> bool {
      target.val = false;
      true
    }
  }

  #[test]
  fn test() {
    let mut target = Target::new();
    let mut invoker = Invoker::new(&mut target);

    assert_eq!(false, invoker.get_target().get());

    invoker.add(UpdateTrueTask);
    assert!(invoker.execute());
    assert_eq!(true, invoker.get_target().get());

    invoker.add(UpdateFalseTask);
    assert!(invoker.execute());
    assert_eq!(false, invoker.get_target().get());
  }
}
