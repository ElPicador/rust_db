use row::insert_to_row;
use row::Row;

pub fn prepare_statement(input: &String) -> Result<Statement, PrepareError> {
    if input.starts_with("insert") {
        return match insert_to_row(input) {
            Some(r) => Ok(Statement { stype: StatementType::Insert, row: r }),
            None => Err(PrepareError::SyntaxError),
        };
    }

    if input.starts_with("select") {
        return Ok(Statement {
            stype: StatementType::Select,
            row: Row {
                id: 0,
                username: String::new(),
                email: String::new(),
            }
        });
    }

    Err(PrepareError::UnrecognizedStatement)
}

pub fn execute_statement(statement: Statement) {
    match statement.stype {
        StatementType::Insert =>
            println!("This is where we would do an insert."),
        StatementType::Select =>
            println!("This is where we would do a select."),
    }
}


pub enum PrepareError {
    UnrecognizedStatement,
    SyntaxError
}

pub enum StatementType {
    Insert,
    Select,
}

pub struct Statement {
    stype: StatementType,
    row: Row,
}
