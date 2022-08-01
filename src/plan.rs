use std::{fmt::Display, path::Path};

use serde::{
  self,
  de::{self, Visitor},
  Deserialize, Deserializer, Serialize,
};

use crate::filesystem::FileSystem;

pub struct Plan {
  root: PlanRoot,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PlanRootV1 {
  version: u32,
  platform: String,
  path: String,
  plan: Vec<PlanElementV1>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PlanElementV1 {
  src: String,
  dest: String,
  id: String,
}

impl PlanRootV1 {
  pub fn new(path: &Path) -> Self {
    Self {
      version: 1,
      platform: Plan::get_target().to_string(),
      path: path.canonicalize().unwrap().to_string_lossy().to_string(),
      plan: vec![],
    }
  }

  pub fn get_version(&self) -> u32 {
    self.version
  }

  pub fn get_platform(&self) -> &str {
    &self.platform
  }

  pub fn get_path(&self) -> &str {
    &self.path
  }

  pub fn get_plans(&self) -> &Vec<PlanElement> {
    &self.plan
  }
}

impl PlanElementV1 {
  pub fn new(src: &String, dest: &String, id: &String) -> Self {
    Self {
      src: src.to_string(),
      dest: dest.to_string(),
      id: id.to_string(),
    }
  }

  pub fn get_source(&self) -> &str {
    &self.src
  }

  pub fn get_destination(&self) -> &str {
    &self.dest
  }

  pub fn get_id(&self) -> &str {
    &self.id
  }
}

pub type PlanRoot = PlanRootV1;
pub type PlanElement = PlanElementV1;

impl Plan {
  pub fn new(path: &Path) -> Self {
    let p = if path.is_file() {
      path.ancestors().next()
    } else {
      Some(path)
    };
    if p.is_none() || !p.unwrap().exists() {
      panic!(
        "path is not exists.",
        // format!("path is not exists. path: {}", path.display())
      );
    }

    Self {
      root: PlanRoot::new(p.unwrap()),
    }
  }

  fn get_target() -> &'static str {
    #[cfg(target_os = "windows")]
    return "windows";
    #[cfg(target_os = "linux")]
    return "linux";
  }

  pub fn add(&mut self, src: &Path, dest: &String) -> Result<(), String> {
    if !src.exists() {
      return Err(format!("{} not found.", src.display()));
    }

    let p = Path::new(&self.root.path).join(src);
    let id = FileSystem::get_id_by_filename(&p);

    if id.is_err() {
      return Err(id.unwrap_err());
    }

    self.root.plan.push(PlanElement {
      src: src.file_name().unwrap().to_string_lossy().to_string(),
      dest: dest.to_string(),
      id: id.unwrap(),
    });

    Ok(())
  }

  pub fn get_root(&self) -> &PlanRoot {
    &self.root
  }

  pub fn get_plans(&self) -> &Vec<PlanElement> {
    &self.root.plan
  }

  pub fn serialize(self) -> Result<String, String> {
    let json = serde_json::to_string(&self.root);
    match json {
      Ok(s) => return Ok(s),
      Err(e) => return Err(format!("error at line: {}, col: {}", e.line(), e.column())),
    }
  }

  pub fn deserialize(data: &String) -> Result<Plan, String> {
    let res = serde_json::from_str::<PlanRoot>(&data);
    match res {
      Ok(p) => {
        let mut plan = Plan::new(Path::new(&p.path));
        plan.root = p;
        return Ok(plan);
      }
      Err(e) => return Err(format!("error at line: {}, col: {}", e.line(), e.column())),
    }
  }
}

#[cfg(test)]
mod tests {
  use std::path::Path;

  use super::Plan;

  #[test]
  fn test() {
    let mut p = Plan::new(Path::new("."));

    let add_result = p.add(Path::new("README.md"), &"RENAMED.md".to_string());
    assert!(add_result.is_ok());

    let serialized = p.serialize().unwrap();
    println!("serialized JSON:");
    println!("{}", serialized);
    assert!(serialized.starts_with("{\"version\":1,\"platform\":"));
    assert!(serialized.contains("\"src\":\"README.md\""));
    assert!(serialized.contains("\"dest\":\"RENAMED.md\""));

    let p2 = Plan::deserialize(&serialized).unwrap();
    assert_eq!(p2.get_root().version, 1);
    assert_eq!(p2.get_root().plan.len(), 1);
    assert_eq!(p2.get_root().plan.get(0).unwrap().src, "README.md");
    assert_eq!(p2.get_root().plan.get(0).unwrap().dest, "RENAMED.md");
  }
}
