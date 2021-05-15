use inflector::Inflector;

use crate::query_parsing::Variable;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ArgumentStruct<'schema> {
    name: String,
    fields: Vec<ArgumentStructField<'schema>>,
}

impl<'schema> ArgumentStruct<'schema> {
    pub fn new(name: String, fields: Vec<ArgumentStructField<'schema>>) -> Self {
        ArgumentStruct { name, fields }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ArgumentStructField<'schema> {
    Variable(Variable<'schema>),
    NestedStruct(String),
}

impl<'schema> ArgumentStructField<'schema> {
    fn name(&self) -> String {
        match self {
            ArgumentStructField::Variable(var) => var.name.to_string().to_snake_case(),
            ArgumentStructField::NestedStruct(type_name) => type_name.to_snake_case(),
        }
    }

    fn type_spec(&self) -> String {
        match self {
            ArgumentStructField::Variable(var) => var.value_type.type_spec().to_string(),
            ArgumentStructField::NestedStruct(type_name) => type_name.clone(),
        }
    }
}

impl std::fmt::Display for ArgumentStruct<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use super::indented;
        use std::fmt::Write;

        writeln!(f, "#[derive(cynic::FragmentArguments, Debug)]")?;
        writeln!(f, "pub struct {} {{", self.name)?;

        for field in &self.fields {
            write!(indented(f, 4), "{}", field)?;
        }
        writeln!(f, "}}")
    }
}

impl std::fmt::Display for ArgumentStructField<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "pub {}: {},", self.name(), self.type_spec())
    }
}
