use crate::ast::{Program, Statement, StepId};

pub fn generate_c_code(program: &Program) -> String {
    let mut c_code = String::from("#include <stdio.h>\n\nint main() {\n");

    for stmt in &program.statements {
        match stmt {
            Statement::Start => {
                c_code.push_str("    // Program Start\n");
            }
            Statement::End => {
                c_code.push_str("    // Program End\n    return 0;\n");
            }
            Statement::StatementChecker(text) => {
                c_code.push_str(&format!("    printf(\"{}\\n\");\n", text));
            }
            Statement::IfThen { id, condition, .. } => {
                c_code.push_str(&format!("    step_{}:\n", id));
                c_code.push_str(&format!("    if ({}) {{\n", condition));
            }
            Statement::ReturnTo(target) => {
                if let StepId::Number(n) = target {
                    c_code.push_str(&format!("    goto step_{};\n", n));
                }
            }
        }
    }

    c_code.push_str("}\n");
    c_code
}