use crate::ast::{Program, Statement, StepId};
use crate::lexer::Token;

pub fn parse(tokens: &[Token]) -> Result<Program, String> {
    let mut statements = Vec::new();
    let mut has_start = false;
    let mut has_end = false;

    for token in tokens {
        match token {
            Token::Start(_) => {
                has_start = true;
                statements.push(Statement::Start);
            }
            Token::End(_) => {
                has_end = true;
                statements.push(Statement::End);
            }
            Token::Statement { text, .. } => {
                statements.push(Statement::StatementChecker(text.clone()));
            }
            Token::If { id, condition, pos } => {
                if condition.is_empty() {
                    return Err(format!("Syntax Error on line {}: 'if{}' is missing a condition.", pos.line, id));
                }
                statements.push(Statement::IfThen {
                    id: *id,
                    condition: condition.clone(),
                    body: vec![],
                });
            }
            Token::ReturnTo { target, .. } => {
                let step_id = if let Ok(num) = target.parse::<usize>() {
                    StepId::Number(num)
                } else {
                    StepId::Named(target.clone())
                };
                statements.push(Statement::ReturnTo(step_id));
            }
            _ => {}
        }
    }

    if !has_start {
        return Err("Compiler Error: Missing 'start' block in .algo source.".to_string());
    }
    if !has_end {
        return Err("Compiler Error: Missing 'end' block in .algo source.".to_string());
    }

    Ok(Program { statements })
}