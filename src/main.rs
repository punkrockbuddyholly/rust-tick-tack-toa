use std::fmt;
// use std::io;
// use rand::Rng;

struct Row {
    position: RowPosition,
    values: RowValues,
}

struct RowValues (Value, Value, Value);

enum Value {
    Naught,
    Cross
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Value::Naught => write!(f, "O"),
            Value::Cross => write!(f, "X"),
        }
    }
}

enum RowPosition {
    Top,
    Middle,
    Bottom,
}

fn print_row_border() {
    println!("+----------+----------+----------+");
}

fn print_row_middle(values: RowValues) {
    println!("|          |          |          |");
    println!("|     {}    |     {}    |     {}    |", values.0, values.1, values.2);
    println!("|          |          |          |");
}

fn print_row(row: Row) {
    match row.position {
        RowPosition::Top => { 
            print_row_border();
            print_row_middle(row.values);
        },
        RowPosition::Middle => {
            print_row_border();
            print_row_middle(row.values);
            print_row_border();
        },
        RowPosition::Bottom => {
            print_row_middle(row.values);
            print_row_border();
        }
    }
}

fn print_grid(_size: &i32) {
    let top_row = Row {
        position: RowPosition::Top,
        values: RowValues(Value::Cross, Value::Naught, Value::Cross),
    };
    let middle_row = Row {
        position: RowPosition::Middle,
        values: RowValues(Value::Naught, Value::Cross, Value::Naught),
    };
    let bottom_row = Row {
        position: RowPosition::Bottom,
        values: RowValues(Value::Cross, Value::Naught, Value::Cross),
    };
    print_row(top_row);
    print_row(middle_row);
    print_row(bottom_row);
}

fn main() {
    let value = "1";
    let value: i32 = value.parse::<i32>().unwrap();
    print_grid(&value);
}
