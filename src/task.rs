trait Task<T> {
  fn execute(&self, target: &mut T) -> bool;
}

struct Invoker<'a, T: 'a> {
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
  use std::fs;
  use super::*;

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
// use uuid::Uuid;
//
// type Callback = dyn FnMut(&str, &str) -> bool;
//
// pub trait Task {
//   fn new<F>(callback: F) -> Self
//   where
//     F: FnMut(&str, &str) -> bool;
//   fn execute(&mut self);
// }
//
// pub trait TaskObjectTrait {}
//
// pub struct TaskBase<F>
// where
//   F: FnMut(&str, &str) -> bool,
// {
//   uuid: String,
//   name: String,
//   callback: Box<F>,
// }
//
// impl<F> TaskBase<F>
// where
//   F: FnMut(&str, &str) -> bool,
// {
//   fn new(name: &str, callback: F) -> Self {
//     Self {
//       uuid: Uuid::new_v4().to_string(),
//       name: name.to_string(),
//       callback: Box::new(callback), // callback: Box::new(|_: &str, _: &str| true),
//     }
//   }
//
//   pub fn get_uuid(&self) -> &str {
//     &self.uuid
//   }
//
//   pub fn get_name(&self) -> &str {
//     &self.name
//   }
// }
//
// pub struct TaskController {
//   tasks: Vec<TaskBase<dyn FnMut(&str, &str) -> bool>>,
// }
//
// impl TaskController {
//   pub fn add(&mut self, task: TaskBase<dyn FnMut(&str, &str) -> bool>) {
//     self.tasks.push(task);
//   }
//
//   pub fn find(&self, id: &str) -> Option<&TaskBase<dyn FnMut(&str, &str) -> bool>> {
//     for t in &self.tasks {
//       if t.get_uuid() == id {
//         return Some(t);
//       }
//     }
//     None
//   }
// }
//
// pub struct NopTask {
//   base: TaskBase<dyn FnMut(&str, &str) -> bool>,
// }
//
// impl NopTask {
//   fn get_base(&self) -> &TaskBase<dyn FnMut(&str, &str) -> bool> {
//     &self.base
//   }
// }
//
// impl Task for NopTask {
//   fn new(callback: dyn FnMut(&str, &str) -> bool) -> Self {
//     Self {
//       base: TaskBase::new("NopTask", callback),
//     }
//   }
//
//   fn execute(&mut self) -> bool {
//     let base = &self.base;
//     let uuid = base.get_uuid();
//     let name = base.get_name();
//     (self.base.callback)(uuid, name)
//   }
// }
//
// // pub struct RenameTask {
// //   base: TaskBase,
// // }
// //
// // impl RenameTask {
// //   fn get_base(&self) -> &TaskBase {
// //     &self.base
// //   }
// // }
// //
// // impl Task for RenameTask {
// //   fn new(callback: dyn FnMut(&str, &str) -> bool) -> Self {
// //     Self {
// //       base: TaskBase::new("RenameTask", callback),
// //     }
// //   }
// //
// //   fn execute(&mut self) -> bool {
// //     todo!()
// //   }
// // }
//
// #[cfg(test)]
// mod tests {
//   use super::*;
//
//   struct Empty;
//   pub struct Simple {
//     val: bool,
//   }
//
//   impl Simple {
//     pub fn new() -> Self {
//       Self { val: false }
//     }
//
//     pub fn set_true(&mut self) {
//       self.val = true
//     }
//
//     pub fn ret_true(&self) -> bool {
//       true
//     }
//   }
//
//   #[test]
//   fn test() {
//     let val = false;
//     let mut callback = Box::new(|_: &str, _: &str| true);
//
//     let mut task = NopTask::new(callback);
//     assert_eq!(true, task.execute());
//     assert_eq!("NopTask", task.get_base().get_name());
//     print!("{}", task.get_base().get_uuid());
//   }
//
//   // fn test_rename_task() {
//   //   let mut simple = Simple::new();
//   //   let mut task = RenameTask::new();
//   //   assert_eq!(false, simple.val);
//   //   assert_eq!(true, task.execute());
//   //   assert_eq!(true, simple.val);
//   //   assert_eq!("RenameTask", task.get_base().get_name());
//   // }
// }
//
// //   N E K O  N O
// //     H I T A I  D E
// //       A S O B U
// //                .___
// //  FAN ART       |  /-,
// //      ,=.__,=.  | / /
// //    .:|_|--|_|=v:!;:.
// //   :::^::::::::::::::.
// //  .::/ `:::"::::::!:::.
// //  ::| __":/__::::::`::
// //  :'|' @ '  @` :::',:'
// //    /""       ""::,'
// //    `.__`--'___,:r
// //     / `/-`./  `.
// //    |   / |      |
// //   ,   ,  ,--, ,---
// //   |`. |  |--| |---
// //   |  `|  |  | |___
//
