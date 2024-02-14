use foundry_compilers::artifacts::Ast;

// Base trait for detectors
pub trait Detector {
    fn title(&self) -> String;
    fn description(&self) -> String;
    fn detect(&self, ast: &Ast);
}

pub mod nc;


// Implement filter for serverity
pub fn run_all_detectors(code: &String, ast: &Ast) {
    nc::run_detectors(code, ast);
}