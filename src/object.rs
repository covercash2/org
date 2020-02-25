
#[derive(Debug)]
pub enum OrgObject<'t> {
	Header(Header<'t>, Vec<OrgObject<'t>>),
	Text(&'t str),
}

#[derive(Debug)]
pub struct Header<'t> {
	level: usize,
	title: &'t str,
	status: Option<&'t str>,
}

impl<'t> Header<'t> {
	pub fn new_root() -> Header<'t> {
		Header {
			level: 0,
			title: "root",
			status: None,
		}
	}

	pub fn header(level: usize, title: &'t str) -> Header<'t> {
		return Header {
			level: level,
			title: title,
			status: None,
		};
	}

	pub fn todo(level: usize, status: &'t str, title: &'t str) -> Header<'t> {
		return Header {
			level: level,
			status: Some(status),
			title: title,
		};
	}

	pub fn level(&self) -> usize {
		self.level
	}
}
