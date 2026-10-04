#[derive(Debug, PartialEq, Clone)]
pub struct Position {
    pub line: usize,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Start(Position),
    End(Position),
    If { id: usize, condition: String, pos: Position },
    Then { id: usize, pos: Position },
    ReturnTo { target: String, pos: Position },
    Statement { text: String, pos: Position },
    LBrace(Position),
    RBrace(Position),
    Unknown { text: String, pos: Position },
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    for (line_idx, line) in input.lines().enumerate() {
        let line_num = line_idx + 1;
        let pos = Position { line: line_num };
        let trimmed = line.trim();

        if trimmed.is_empty() || trimmed.starts_with("//") {
            continue;
        }

        if trimmed.eq_ignore_ascii_case("start") {
            tokens.push(Token::Start(pos));
        } else if trimmed.eq_ignore_ascii_case("end") {
            tokens.push(Token::End(pos));
        } else if trimmed.starts_with("if") && trimmed.contains('{') {
            if let Some(brace_pos) = trimmed.find('{') {
                let close_pos = trimmed.find('}').unwrap_or(trimmed.len());
                let id_str = trimmed[2..brace_pos].trim();
                let id = id_str.parse::<usize>().unwrap_or(0);
                let condition = trimmed[brace_pos + 1..close_pos].trim().to_string();

                tokens.push(Token::If { id, condition, pos });
            }
        } else if trimmed.starts_with("return to") {
            let target = trimmed.trim_start_matches("return to").trim().trim_matches('"');
            tokens.push(Token::ReturnTo { target: target.to_string(), pos });
        } else if trimmed == "}" {
            tokens.push(Token::RBrace(pos));
        } else {
            tokens.push(Token::Statement { text: trimmed.to_string(), pos });
        }
    }

    tokens
}