use std::fmt::Write;

use console::{Color, style};

use crate::conclusion::Conclusion;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum Value {
    Bool(bool),
    String(String),
    Object(Object),
    Vector(Vec<Value>),
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub(crate) enum FieldStyle {
    #[default]
    Auto,
    Enum,
    Price,
    Description,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Object {
    fields: Vec<Field>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct ObjectBuilder {
    object: Object,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct Field {
    pub(crate) key: String,
    pub(crate) value: Value,
    pub(crate) style: FieldStyle,
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Bool(value) => write!(f, "{}", value),
            Value::String(value) => write!(f, "{}", value),
            Value::Object(object) => object.fmt(f),
            Value::Vector(values) => {
                write!(f, "[")?;
                for value in values {
                    value.fmt(f)?;
                }
                write!(f, "]")?;

                Ok(())
            }
        }
    }
}

impl std::fmt::Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.pretty_print(0, true, &mut s)?;
        write!(f, "{}", s)
    }
}

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Self::String(value.to_string())
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}

impl From<u8> for Value {
    fn from(value: u8) -> Self {
        Self::String(value.to_string())
    }
}

impl From<u64> for Value {
    fn from(value: u64) -> Self {
        Self::String(value.to_string())
    }
}

impl Object {
    fn pretty_print(&self, indentation: u32, w: bool, f: &mut String) -> std::fmt::Result {
        for field in &self.fields {
            if w {
                write!(f, "{}", "  ".repeat(indentation as usize))?;
            }

            match &field.value {
                Value::Bool(value) => {
                    let conclusion = Conclusion(*value);
                    writeln!(
                        f,
                        "{} {}: {}",
                        style("*").magenta().bold(),
                        field.key,
                        style(conclusion.value()).fg(conclusion.color()).bold()
                    )?;
                }

                Value::String(value) => match &field.style {
                    FieldStyle::Auto => {
                        writeln!(
                            f,
                            "{} {}: {}",
                            style("*").magenta().bold(),
                            field.key,
                            style(&value).blue().bold()
                        )?;
                    }

                    FieldStyle::Enum => {
                        writeln!(
                            f,
                            "{} {}: {}",
                            style("*").magenta().bold(),
                            field.key,
                            style(&value).black().bg(Color::Color256(141)).bold()
                        )?;
                    }

                    FieldStyle::Price => {
                        writeln!(
                            f,
                            "{} {}: {}{}",
                            style("*").magenta().bold(),
                            field.key,
                            style(&value).green().bold(),
                            style("$").green().bold()
                        )?;
                    }

                    FieldStyle::Description => {
                        if value.is_empty() {
                            writeln!(f)?;
                        } else {
                            writeln!(
                                f,
                                "{} {}: {}",
                                style("*").magenta().bold(),
                                field.key,
                                style(&value).bold().bg(Color::Color256(234))
                            )?;
                        }
                    }
                },

                Value::Object(value) => {
                    if w {
                        write!(f, "{} ", style("*").magenta().bold())?;
                    };

                    writeln!(
                        f,
                        "{}: {}",
                        style(&field.key).bold(),
                        style("{").magenta().bold(),
                    )?;

                    value.pretty_print(indentation + 1, true, f)?;
                    write!(f, "{}", "  ".repeat(indentation as usize))?;
                    write!(f, "{}", style("}").magenta().bold())?;

                    if w {
                        writeln!(f)?;
                    }
                }

                // TODO: currently looks bad, need inline format and wide format
                Value::Vector(values) => {
                    write!(
                        f,
                        "{} {}: {}",
                        style("*").magenta().bold(),
                        field.key,
                        style("[").bold()
                    )?;

                    // TODO: remove trailing comma
                    for value in values {
                        if let Value::Object(object) = value {
                            write!(f, "(")?;
                            object.pretty_print(indentation, false, f)?;
                            // write!(f, "{}", "  ".repeat(indentation as usize))?;
                            writeln!(f, "),")?;
                        } else {
                            write!(f, "{}, ", style(&value).yellow().bold())?;
                        }
                    }

                    // TODO: inline with last element
                    writeln!(f, "{}", style("]").bold())?;
                }
            }
        }

        Ok(())
    }
}

impl ObjectBuilder {
    pub(crate) fn with_field(mut self, field: Field) -> Self {
        self.object.fields.push(field);
        self
    }

    pub(crate) fn build(self) -> Object {
        self.object
    }
}

impl Field {
    pub(crate) fn new(key: &str, value: Value) -> Self {
        Self {
            key: key.to_string(),
            value,
            style: FieldStyle::default(),
        }
    }

    pub(crate) fn with_style(mut self, style: FieldStyle) -> Self {
        self.style = style;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::{Field, ObjectBuilder, Value};

    #[test]
    fn object() {
        let object = ObjectBuilder::default()
            .with_field(Field::new("Test", Value::from("hello")))
            .with_field(Field::new("Test 2", Value::Bool(true)))
            .with_field(Field::new(
                "Child",
                Value::Object(
                    ObjectBuilder::default()
                        .with_field(Field::new(
                            "Descendant",
                            Value::Object(
                                ObjectBuilder::default()
                                    .with_field(Field::new(
                                        "Points",
                                        Value::Vector(vec![Value::from("120.510")]),
                                    ))
                                    .build(),
                            ),
                        ))
                        .build(),
                ),
            ))
            .build();

        println!("{}", object.to_string());
    }
}
