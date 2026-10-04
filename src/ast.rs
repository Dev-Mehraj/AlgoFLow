#[derive(Debug, PartialEq, Clone)]
pub enum StepId {
    Named(String),
    Number(usize),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Start,
    End,
    StatementChecker(String),
    IfThen {
        id: usize,
        condition: String,
        body: Vec<Statement>,
    },
    ReturnTo(StepId),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub statements: Vec<Statement>,
}