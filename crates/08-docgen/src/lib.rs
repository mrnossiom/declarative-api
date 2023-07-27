mod types;

use std::{
	collections::{BTreeMap, HashMap},
	fs,
	path::Path,
};

static CODEBLOCK_LANGUAGE: &str = "c"; // ada, applescript, c are all candidates

pub trait GenerateFilelist {
	fn markdown(&self, output_files: &mut HashMap<Box<Path>, String>);
}

pub trait Generate {
	fn markdown(&self) -> String;
}

pub trait WriteFiles {
	fn write(&self, root_path: &Path);
}

impl Generate for hir_lowering::types::ApiMetadata {
	fn markdown(&self) -> String {
		format!(
			"# {} (v{})\n## URLs\n{}\n{}\n",
			self.name,
			self.version,
			self.urls.format_as_list(),
			format_optional(&self.comment)
		)
	}
}

impl Generate for hir_lowering::types::Response {
	fn markdown(&self) -> String {
		format!(
			"{}\n##### Headers\n{}\n##### Body\n{}",
			format_optional(&self.comment),
			self.headers.markdown(),
			self.body.markdown(),
		)
	}
}

impl Generate for hir_lowering::types::Method {
	fn markdown(&self) -> String {
		format!(
			"{}\n##### Query parameters\n{}\n##### Headers\n{}\n### Responses\n\n{}",
			format_optional(&self.comment),
			self.query_params.markdown(),
			self.headers.markdown(),
			self.responses.markdown(),
		)
	}
}

impl Generate for hir_lowering::types::Endpoint {
	fn markdown(&self) -> String {
		self.methods.markdown()
	}
}

impl Generate for hir_lowering::types::Model {
	fn markdown(&self) -> String {
		format!(
			"{}\n{}",
			format_optional(&self.comment),
			self.model_body.markdown(),
		)
	}
}

impl Generate for hir_lowering::types::Scope {
	fn markdown(&self) -> String {
		format!(
			"{}\n# Child scopes\n{}\n\n# Endpoints\n{}\n# Models\n{}\n",
			format_optional(&self.comment),
			Vec::from_iter(self.child_scopes.clone())
				.iter()
				.map(|x| format!("[{}]({})", x, vec![x.clone()].get_markdown_file_name()))
				.collect::<Vec<String>>()
				.format_as_list(),
			self.endpoints.markdown(),
			self.models.markdown()
		)
	}
}

impl GenerateFilelist for hir_lowering::types::IntermediateRepresentation {
	fn markdown(&self, output_files: &mut HashMap<Box<Path>, String>) {
		output_files.insert(
			Path::new("index.md").into(),
			format!(
				"{}\n{}",
				self.metadata.markdown(),
				self.scopes
					.get(&hir_lowering::types::ScopePath::new())
					.expect("couldn't find root scope")
					.markdown()
			),
		);
		for (scope_path, scope) in &self.scopes {
			if scope_path != &hir_lowering::types::ScopePath::new() {
				let parent_file_link = match scope_path.len() {
					1 => "## Index\n[index](../index.md)".into(),
					_ => {
						let parent_file = &scope_path[scope_path.len() - 2];
						format!("## Parent\n[{}](../{}.md)\n", &parent_file, &parent_file)
					}
				};
				output_files.insert(
					Path::new(scope_path.get_markdown_file_name().as_str()).into(),
					format!(
						"# {}\n{}\n{}",
						scope_path.get_name(),
						parent_file_link,
						scope.markdown()
					),
				);
			}
		}
	}
}

impl<T: Generate> Generate for HashMap<String, T> {
	fn markdown(&self) -> String {
		let mut v = vec![];
		for (key, value) in self {
			v.push(format!("## {key}\n{}", value.markdown()));
		}
		v.join("\n\n")
	}
}

impl<T: Generate> Generate for BTreeMap<String, T> {
	fn markdown(&self) -> String {
		let mut v = vec![];
		for (key, value) in self {
			v.push(format!("### {key}\n{}", value.markdown()));
		}
		v.join("\n\n")
	}
}

impl<T: Generate> Generate for BTreeMap<u32, T> {
	fn markdown(&self) -> String {
		let mut v = vec![];
		for (key, value) in self {
			v.push(format!("#### {key}\n{}", value.markdown()));
		}
		v.join("\n\n")
	}
}

impl Generate for BTreeMap<String, hir_lowering::types::KeyValuePairValue> {
	fn markdown(&self) -> String {
		let mut v = vec![];
		for (key, value) in self {
			v.push(format!("{}{}: {}", hir_lowering::INDENT, key, value,));
		}
		format!("```{}\n{{\n{}\n}}\n```", CODEBLOCK_LANGUAGE, v.join("\n"))
	}
}

impl WriteFiles for HashMap<Box<Path>, String> {
	fn write(&self, root_path: &Path) {
		for (relative_file_path, file_content) in self {
			let file_path = root_path.join(relative_file_path);
			if let Some(p) = file_path.parent() {
				fs::create_dir_all(p).expect("couldn't create a dir");
			};
			fs::write(file_path, file_content).expect("couldn't write to a file");
		}
	}
}

fn format_optional(optional: &Option<String>) -> String {
	match optional.clone() {
		Some(string) => format!("\n{}", string),
		None => String::new(),
	}
}

trait FormatAsList {
	fn format_as_list(&self) -> String;
}
impl FormatAsList for &Vec<String> {
	fn format_as_list(&self) -> String {
		if !self.is_empty() {
			return format!("- {}", self.join("\n- "));
		}
		String::new()
	}
}

impl FormatAsList for Vec<String> {
	fn format_as_list(&self) -> String {
		(&self).format_as_list()
	}
}

trait Naming {
	fn get_name(&self) -> String;

	fn get_path_name(&self) -> String;

	fn get_markdown_file_name(&self) -> String;
}

impl Naming for hir_lowering::types::ScopePath {
	fn get_name(&self) -> String {
		self.join(".")
	}
	fn get_path_name(&self) -> String {
		format!(
			"{}/{}",
			self.join("/"),
			self.last().expect("couldn't get last element in array")
		)
	}

	fn get_markdown_file_name(&self) -> String {
		format!("{}.md", self.get_path_name())
	}
}

#[cfg(test)]
mod tests;
