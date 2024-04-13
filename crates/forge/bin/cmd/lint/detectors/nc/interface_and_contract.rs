use crate::cmd::lint::utils::get_full_line;

use super::super::Detector;
use foundry_compilers::artifacts::{Ast, Node, NodeType};


pub struct InterfaceAndContractDetector {
    code: String,
    // TODO: add issues to return if report will be dumped to a file
}

impl InterfaceAndContractDetector {
    pub fn new(code: &String) -> InterfaceAndContractDetector {
        InterfaceAndContractDetector {
            code: code.clone(),
        }
    }
}

impl Detector for InterfaceAndContractDetector {
    fn title(&self) -> String {
        String::from("Interfaces and Contracts in the Same File")
    }

    fn description(&self) -> String {
        String::from("Interfaces should be declared in a separate file and not in the same file where the contract resides. This helps in maintaining a clean and organized codebase.")
    }

    fn detect(&self, ast: &Ast) {
        let mut interface = None;
        let mut contract = None;

        for node in ast.nodes.iter() {
            if NodeType::ContractDefinition == node.node_type {
                if let Some(kind) = node.other.get("contractKind").and_then(|v| v.as_str()) {
                    match kind {
                        "interface" => interface = Some(node),
                        "contract" => contract = Some(node),
                        _ => {}
                    }
                }
            }
        }
        if interface.is_some() && contract.is_some() {
            self.print_issue_in_console(interface.unwrap(), contract.unwrap());
        }
    }

    fn print_issue_in_console(&self, interface: &Node, contract: &Node) {
        println!("{}\n", self.title());
        println!("{}", get_full_line(&self.code, interface.src.start));
        println!("{}", get_full_line(&self.code, contract.src.start));
    }
}