use super::{HasProfileKey, Pid};
use enum_iterator::IntoEnumIterator;

#[derive(Clone, Copy, Debug, IntoEnumIterator, PartialEq)]
pub enum DigitalObjectType {
    // a collection (api) item with all related objects to replicate the results.
    Publication,
    // a written piece that explains a publication (human readable).
    Paper,
    // an object describing an algorithm. Not self runnable.
    // i.e. a script for bash/python, pseudocode, ...
    Algorithm,
    // an application may interpret algorithms or is compatible to certain data formats.
    Application,
    // a collection (api) item collecting ManuscriptPages.
    Manuscript,
    // a manuscript page of any format with any kind of additional metadata.
    ManuscriptPage,
    // one or multiple annotations belonging all to the same unit.
    // might be a collection (api) item.
    Annotations,
}

impl HasProfileKey for DigitalObjectType {
    fn get_key() -> Pid {
        Pid("21.T11148/c83481d4bf467110e7c9".into())
    }

    fn get_key_name() -> &'static str {
        "digitalObjectType"
    }
}

/// Associates types with their PID.
/// FIXME PIDs are not yet correct as they do not yet exist.
impl From<DigitalObjectType> for Pid {
    fn from(p: DigitalObjectType) -> Self {
        match p {
            DigitalObjectType::Publication => Pid(r#"21.T11148/61fd3446879407065218"#.into()),
            DigitalObjectType::Paper => Pid(r#"21.T11148/61fd3446879407065218"#.into()),
            DigitalObjectType::Algorithm => Pid(r#"21.T11148/61fd3446879407065218"#.into()),
            DigitalObjectType::Application => Pid(r#"21.T11148/61fd3446879407065218"#.into()),
            DigitalObjectType::Manuscript => Pid(r#"21.T11148/61fd3446879407065218"#.into()),
            DigitalObjectType::ManuscriptPage => Pid(r#"21.T11148/61fd3446879407065218"#.into()),
            DigitalObjectType::Annotations => Pid(r#"21.T11148/61fd3446879407065218"#.into()),
            // IMPORTANT: DO NOT DO A CATCH-ALL CASE HERE!
        }
    }
}
