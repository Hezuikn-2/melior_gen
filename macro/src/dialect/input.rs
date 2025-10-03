mod input_field;

use self::input_field::InputField;
use std::ops::Deref;

pub struct DialectInput {
    pub name: String,
    // TODO Remove this field.
    pub table_gen: Option<String>,
    // TODO Remove this field.
    pub td_file: Option<String>,
    // TODO Remove this field.
    pub include_directories: Vec<String>,
    pub files: Vec<String>,
    pub directories: Vec<String>,
}

impl DialectInput {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn table_gen(&self) -> Option<&str> {
        self.table_gen.as_deref()
    }

    pub fn td_file(&self) -> Option<&str> {
        self.td_file.as_deref()
    }

    pub fn include_directories(&self) -> impl Iterator<Item = &str> {
        self.include_directories.iter().map(Deref::deref)
    }

    pub fn files(&self) -> impl Iterator<Item = &str> {
        self.files.iter().map(Deref::deref)
    }

    pub fn directories(&self) -> impl Iterator<Item = &str> {
        self.directories.iter().map(Deref::deref)
    }
}
