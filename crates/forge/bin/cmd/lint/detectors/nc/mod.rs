use super::Detector;
use foundry_compilers::artifacts::Ast;

mod interface_and_contract;

pub fn run_detectors(code: &String, ast: &Ast) {
    let detectors: Vec<Box<dyn Detector>> = vec![
        Box::new(interface_and_contract::InterfaceAndContractDetector::new(code)),
    ];
    
    for detector in detectors {
        detector.detect(ast);
    }
}