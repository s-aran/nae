use task;

struct VersionInfoTask;
impl Task<Target> for VersionInfoTask {
  fn execute(&self, target: &mut Target) -> bool {
    println!("N E K O  N O");
    println!("    H I T A I  D E");
    println!("      A S O B U");
    println!("               .___");
    println!(" FAN ART       |  /-,");
    println!("     ,=.__,=.  | / /    ");
    println!("   .:|_|--|_|=v:!;:.");
    println!("  :::^::::::::::::::.");
    println!(" .::/ `:::\"::::::!:::.");
    println!(" ::| __\":/__::::::`:: ");
    println!(" :'|' @ '  @` :::',:'");
    println!("   /\"\"       \"\"::,'");
    println!("   `.__`--'___,:r");
    println!("    / `/-`./  `.");
    println!("   |   / |      |");
    println!("  ,   ,  ,--, ,---");
    println!("  |`. |  |--| |---");
    println!("  |  `|  |  | |___");

    true
  }
}
