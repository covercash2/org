use crate::{error, status_labels::StatusLabels};

pub struct Config {
    pub file_path: String,
    pub status_labels: StatusLabels,
}

pub struct Builder {
    file_path: Option<String>,
    status_labels: Option<StatusLabels>,
}

impl Builder {
    pub fn build(self) -> error::Result<Config> {
        let file_path = match self.file_path {
            Some(path) => Ok(path),
            // TODO config error?
            None => Err(error::OrgError::unexpected("did not receive file path")),
        }?;

        let status_labels = self.status_labels.unwrap_or_default();

        Ok(Config {
            file_path,
            status_labels,
        })
    }

    pub fn file_path(mut self, path: String) -> Self {
        self.file_path.replace(path);
        self
    }

    pub fn status_labels(mut self, labels: StatusLabels) -> Self {
        self.status_labels.replace(labels);
        self
    }
}
