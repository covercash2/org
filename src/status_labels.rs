use crate::error;

const STATUS_LABELS: [&'static str; 3] = ["TODO", "STARTED", "DONE"];

/// an owned string that represents status labels
pub struct StatusLabels(String);

impl StatusLabels {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.0.split(',')
    }
}

impl Default for StatusLabels {
    fn default() -> Self {
        StatusLabels::from(STATUS_LABELS.as_ref())
    }
}

impl std::str::FromStr for StatusLabels {
    type Err = error::OrgError;

    fn from_str(s: &str) -> error::Result<Self> {
        // check string for bad characters
        if s.split(',').all(|s| s.chars().all(char::is_alphanumeric)) {
            Ok(StatusLabels(String::from(s)))
        } else {
            Err(error::OrgError::Unexpected(format!(
                "unexpected characters in status labels:\n\t{}",
                s
            )))
        }
    }
}

impl From<&[&'static str]> for StatusLabels {
    fn from(labels: &[&'static str]) -> Self {
        StatusLabels(labels.join(",").into())
    }
}
