use std::{env, fs};

use cri_table::table::{CriTable, TableJsonWrapper, UTF_SIGNATURE};
use similar::{ChangeTag, TextDiff};

pub fn print_table(table: &CriTable) -> String {
    serde_json::to_string_pretty(&TableJsonWrapper::Table(table.to_json())).unwrap()
}

fn main() {
    let mut args = env::args().skip(1);
    let file_path = args.next().unwrap();
    let file_content = fs::read(&file_path).unwrap();
    let original_table = CriTable::read(&file_content, None, UTF_SIGNATURE).unwrap();
    let recreated_table_content = original_table.0.write(&original_table.1).unwrap();
    fs::write(format!("{}.recreated", file_path), &recreated_table_content).unwrap();
    let recreated_table = CriTable::read(&recreated_table_content, None, UTF_SIGNATURE).unwrap();

    let diff = TextDiff::from_lines(
        print_table(&original_table.0),
        print_table(&recreated_table.0),
    );

    let mut result = Vec::with_capacity(diff.iter_all_changes().count());

    for change in diff.iter_all_changes() {
        match change.tag() {
            ChangeTag::Delete => result.push(format!("{}{}{}", "-", change.old_index().unwrap(), change)),
            ChangeTag::Insert => result.push(format!("{}{}{}", "+", change.new_index().unwrap(), change)),
            ChangeTag::Equal => (),
        };
    }

    fs::write(format!("{}.diff", file_path), result.join("")).unwrap();
}