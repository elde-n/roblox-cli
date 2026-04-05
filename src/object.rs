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

macro_rules! impl_from_primitives_for_value {
    ($($t:ty),* $(,)?) => {
        $(
            impl From<$t> for Value {
                fn from(v: $t) -> Self {
                    Value::String(v.to_string())
                }
            }
        )*
    };
}

impl_from_primitives_for_value!(
    u8, u16, u32, u64, usize, i8, i16, i32, i64, isize, f32, f64, &str, String
);

impl From<bool> for Value {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl From<Object> for Value {
    fn from(value: Object) -> Self {
        Self::Object(value)
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(value: Vec<T>) -> Self {
        Self::Vector(value.into_iter().map(Into::into).collect())
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

mod object {
    #[macro_export]
    macro_rules! object {
        {} => ($crate::object::ObjectBuilder::default().build());

        ( $( $f:tt ),* $(,)? ) => {{
            let mut builder = $crate::object::ObjectBuilder::default();
            $(
                builder = builder.with_field(object!(@field $f));
            )*
            builder.build()
        }};

        // nested object: ("Key", { (...) })
        (@field ($key:expr, { $( $inner:tt ),* $(,)? } ) ) => {
            $crate::object::Field::new($key, $crate::object::Value::Object(object!( $( $inner ),* )))
        };

        // vec with style: ("Key", vec![ ... ], Style)
        (@field ($key:expr, vec![ $($elem:expr),* $(,)? ], $style:expr) ) => {
            $crate::object::Field::new($key, $crate::object::Value::from(vec![ $( $elem ),* ])).with_style($style)
        };

        // any value with style: ("Key", val, Style)
        (@field ($key:expr, $val:expr, $style:expr) ) => {
            $crate::object::Field::new($key, $crate::object::Value::from($val)).with_style($style)
        };

        // vec without style: ("Key", vec![ ... ])
        (@field ($key:expr, vec![ $($elem:expr),* $(,)? ] ) ) => {
            $crate::object::Field::new($key, $crate::object::Value::from(vec![ $( $elem ),* ]))
        };

        // plain pair: ("Key", val)
        (@field ($key:expr, $val:expr) ) => {
            $crate::object::Field::new($key, $crate::object::Value::from($val))
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::object::FieldStyle;

    #[test]
    fn object() {
        use super::{Field, ObjectBuilder, Value};

        let object = ObjectBuilder::default()
            .with_field(Field::new("Test", Value::from("hello")))
            .with_field(Field::new("Test 2", Value::from(true)))
            .with_field(Field::new(
                "Child",
                Value::from(
                    ObjectBuilder::default()
                        .with_field(Field::new(
                            "Descendant",
                            Value::from(
                                ObjectBuilder::default()
                                    .with_field(
                                        Field::new(
                                            "Points",
                                            Value::from(vec![
                                                Value::from("120.510"),
                                                Value::from("test"),
                                                Value::from("1939"),
                                            ]),
                                        )
                                        .with_style(FieldStyle::Enum),
                                    )
                                    .build(),
                            ),
                        ))
                        .build(),
                ),
            ))
            .build();

        println!("{}", object.to_string());
    }

    #[test]
    fn object_macro() {
        use crate::object;
        let object = object!(
            ("Test", "hello"),
            ("Test2", true),
            ("Child", {
                ("Descendant", {
                    ("Points", vec!["120.510", "test", "1939"], FieldStyle::Enum)
                })
            })
        );

        println!("{}", object.to_string());
    }
}
