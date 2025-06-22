use std::{collections::HashMap, io::stdin};
use serde_derive::{Serialize, Deserialize};
use std::fs::{self, File};
use std::io::Write;

#[derive(Debug,Serialize,Deserialize)]
struct Table {
    name : String,
    columns : Vec<String>,
    rows : Vec<HashMap<String,String>>, 
}

#[derive(Debug, Serialize, Deserialize)]
struct Database {
    tables : HashMap<String,Table>
}

enum Command {
    Create (String),
    Insert (String),
    Select (String),
    Drop (String),
    Exit,
    Help,
}

fn save_to_db(db: &Database){
    match serde_json::to_string_pretty(db){
        Ok(json) =>{
            let mut file = File::create("db.json").expect("Failed to Create");
            file.write_all(json.as_bytes()).expect("Failed to save");
            println!("db.json saved locally");
        }
        Err(e) => {
            println!("Unable to persist the DB locally due to error {}", e);
        }
    }
}

fn load_db_disk() -> Database {
    let filename = "db.json";
    match fs::read_to_string(filename){
        Ok(content) => {
            match serde_json::from_str(&content){
                Ok(db) => {
                    println!("db.json detected");
                    db
                }
                Err(e) => {
                    println!("Error {} parsing the data", e);
                    Database { tables : HashMap::new() }
                }
            }
        }
        Err(_) => {
            println!("No database found locally");
            Database { tables : HashMap::new() } 
        }
    }
}

fn command_parser(input: &str) -> Command {
    let cmd = input.trim().to_lowercase();
    if cmd == "help" {
        Command::Help
    } else if cmd == "exit" {
        Command::Exit
    } else if cmd.starts_with("create") {
        Command::Create(input.trim().to_string())
    } else if cmd.starts_with("insert") {
        Command::Insert(input.trim().to_string())
    } else if cmd.starts_with("select") {
        Command::Select(input.trim().to_string())
    } else if cmd.starts_with("drop") {
        Command::Drop(input.trim().to_string())
    } else {
        Command::Help 
    }
}


fn handle_create(cmd: String, db: &mut Database){
    let parts = cmd.trim().splitn(3, ' ').collect::<Vec<_>>();
    if parts.len() < 3 || !parts[1].eq_ignore_ascii_case("table") {
        println!("Syntax error. Use: CREATE TABLE <name> (<columns>)");
        return;
    }
    let table_name_cols = parts[2];
    let open_para = table_name_cols.find('(');
    let close_para = table_name_cols.find(')');

    if let (Some(start), Some(end)) = (open_para, close_para){
        let table_name = &table_name_cols[..start].trim();
        let col_str = &table_name_cols[start+1..end];
        let columns: Vec<String> = col_str
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let table = Table{
            name : table_name.to_string(),
            columns,
            rows : vec![],
        };
        db.tables.insert(table_name.to_string(), table);
        println!("Table {} created!", table_name);
    } else {
        println!("Invalid Syntax");
        return;
    }
}

fn handle_insert(cmd: String, db: &mut Database) {
    let parts = cmd.trim().splitn(4, ' ').collect::<Vec<_>>();

    if parts.len() < 4 || !parts[1].eq_ignore_ascii_case("into") || !parts[3].to_lowercase().starts_with("values") {
        println!("Syntax error. Use: INSERT INTO <table> VALUES (...)");
        return;
    }

    let table_name = parts[2];
    let values_part = parts[3];
    let open_paren = values_part.find('(');
    let close_paren = values_part.find(')');

    if let (Some(start), Some(end)) = (open_paren, close_paren) {
        let values_str = &values_part[start + 1..end];
        let values: Vec<String> = values_str
            .split(',')
            .map(|v| v.trim().to_string())
            .collect();

        if let Some(table) = db.tables.get_mut(table_name) {
            if values.len() != table.columns.len() {
                println!("Column count does not match value count.");
                return;
            }

            let mut row = HashMap::new();
            for (col, val) in table.columns.iter().zip(values) {
                row.insert(col.clone(), val);
            }

            table.rows.push(row);
            println!("Row inserted into table '{}'", table_name);
        } else {
            println!("Table '{}' does not exist.", table_name);
        }
    } else {
        println!("Invalid syntax in VALUES. Use: INSERT INTO <table> VALUES (...)");
    }
}

fn select_table(cmd: String, db: &Database) {
    let cmd = cmd.trim();
    let lower = cmd.to_lowercase();

    if !lower.starts_with("select ") {
        println!("Syntax error. Use: SELECT <columns> FROM <table>");
        return;
    }

    if let Some((cols_part, table_part)) = cmd[7..].split_once("FROM") {
        let columns: Vec<String> = cols_part
            .trim()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();

        let table_name = table_part.trim();

        if let Some(table) = db.tables.get(table_name) {
            if columns.len() == 1 && columns[0] == "*" {
                println!("Table: {}", table.name);
                println!("{:?}", table.columns);
                let null = String::from("NULL");
                for row in &table.rows {
                    let values: Vec<_> = table.columns.iter().map(|col| row.get(col).unwrap_or(&null)).collect();
                    println!("{:?}", values);
                }
            } else {

                for col in &columns {
                    if !table.columns.contains(col) {
                        println!("Column '{}' not found in table '{}'", col, table.name);
                        return;
                    }
                }

                println!("Table: {}", table.name);
                println!("{:?}", columns);
                let null = String::from("NULL");
                for row in &table.rows {
                    let values: Vec<_> = columns.iter().map(|col| row.get(col).unwrap_or(&null)).collect();
                    println!("{:?}", values);
                }
            }
        } else {
            println!("Table '{}' not found", table_name);
        }
    } else {
        println!("Invalid SELECT syntax. Use: SELECT <columns> FROM <table>");
    }
}

fn drop_table(cmd: String, db: &mut Database){
    let query = cmd.trim().splitn(3, ' ').collect::<Vec<_>>();

    if query.len() < 3 || !query[1].eq_ignore_ascii_case("table") {
        println!("Syntax error. Use: DROP TABLE <table>");
        return;
    }

    let table_name = query[2];
    if db.tables.remove(table_name).is_some() {
        println!("Table '{}' is dropped!", table_name);
    } else {
        println!("No table named '{}' found", table_name);
    }

}

fn exec_command(c : Command, db: &mut Database){
    match c{
        Command::Help => {
            println!("Available commands:");
            println!(" CREATE TABLE <name> (<columns>)");
            println!(" INSERT INTO <table> VALUES (...)");
            println!(" SELECT * FROM <table>");
            println!(" DROP TABLE <table>");
            println!(" EXIT");
        }
        Command::Exit => {
            println!("Exiting...");
            return;
        }
        Command::Create(name) => {
            handle_create(name, db);
        }
        Command::Insert(name) => {
            handle_insert(name, db);
        }
        Command::Select(name) => {
            select_table(name, db);
        }
        Command::Drop(name) => {
            drop_table(name, db);
        }
        _ => {
            println!("Invalid Command");
        }
    }
}

fn main() {
    let mut db = load_db_disk();
    println!("Welcome to RSQL");
    loop {
        let mut user_input = String::new();
        println!("rsql:>");
        stdin().read_line(&mut user_input).expect("Failed");
        let command = command_parser(&user_input);

        if let Command::Exit = command {
            save_to_db(&db);
            exec_command(Command::Exit, &mut db);
            break;
        }

        exec_command(command, &mut db);
    }


}