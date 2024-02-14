use super::super::Detector;
use foundry_compilers::artifacts::{Ast, NodeType};


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
        let mut has_interface = None;
        let mut has_contract = None;

        for node in ast.nodes.iter() {
            if NodeType::ContractDefinition == node.node_type {
                if let Some(kind) = node.other.get("contractKind").and_then(|v| v.as_str()) {
                    match kind {
                        "interface" => has_interface = Some(&node.src),
                        "contract" => has_contract = Some(&node.src),
                        _ => {}
                    }
                }
            }
        }
        if has_interface.is_some() && has_contract.is_some() {
            println!("{}\n{}", self.title(), self.description());
            // TODO: add issues to return if report will be dumped to a file

            // Should be better way to print the issue in code
            // In current AST I can get only start and length of the node
            // or start and length of the name

            // if there no way to extract CodeLocation from cached AST
            // Should use solang-parser to create AST 
        }
    }
}