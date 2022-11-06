#![allow(clippy::needless_lifetimes)]

mod capabilities;
pub mod citation;
mod client;
pub mod component_db;
pub mod db;
mod dispatch;
pub mod distro;
pub mod features;
mod lang_data;
mod line_index;
mod line_index_ext;
mod options;
pub mod parser;
mod server;
pub mod syntax;
pub(crate) mod util;

pub use self::{
    capabilities::ClientCapabilitiesExt,
    lang_data::*,
    line_index::{LineCol, LineColUtf16, LineIndex},
    line_index_ext::LineIndexExt,
    options::*,
    server::Server,
};

#[salsa::jar(db = Db)]
pub struct Jar(
    db::Word,
    db::document::Location,
    db::document::Location_path,
    db::document::Contents,
    db::document::Contents_line_index,
    db::document::LinterData,
    db::document::Document,
    db::document::Document_parse,
    db::document::Document_can_be_index,
    db::document::Document_can_be_built,
    db::parse::TexDocumentData,
    db::parse::TexDocumentData_analyze,
    db::parse::BibDocumentData,
    db::parse::LogDocumentData,
    db::analysis::TexLink,
    db::analysis::label::Number,
    db::analysis::label::Name,
    db::analysis::TheoremEnvironment,
    db::analysis::GraphicsPath,
    db::analysis::TexAnalysis,
    db::dependency::Resolved,
    db::dependency::Implicit,
    db::dependency::Graph,
    db::dependency::Graph_preorder,
    db::workspace::Workspace,
    db::workspace::Workspace_working_dir,
    db::workspace::Workspace_output_dir,
    db::workspace::Workspace_graph,
    db::workspace::Workspace_link_locations,
    db::workspace::Workspace_explicit_links,
    db::workspace::Workspace_implicit_links,
    db::workspace::Workspace_parents,
    db::workspace::Workspace_related,
    db::workspace::Workspace_number_of_label,
    db::diagnostics::tex::collect,
    db::diagnostics::bib::collect,
    db::diagnostics::log::collect,
    db::diagnostics::collect,
    db::diagnostics::collect_filtered,
);

pub trait Db: salsa::DbWithJar<Jar> {}

impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}

#[salsa::db(crate::Jar)]
pub struct Database {
    storage: salsa::Storage<Self>,
}

impl Default for Database {
    fn default() -> Self {
        let storage = salsa::Storage::default();
        let db = Self { storage };
        db::workspace::Workspace::new(
            &db,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        );

        db
    }
}

impl salsa::Database for Database {}

impl salsa::ParallelDatabase for Database {
    fn snapshot(&self) -> salsa::Snapshot<Self> {
        salsa::Snapshot::new(Self {
            storage: self.storage.snapshot(),
        })
    }
}

pub(crate) fn normalize_uri(uri: &mut lsp_types::Url) {
    fn fix_drive_letter(text: &str) -> Option<String> {
        if !text.is_ascii() {
            return None;
        }

        match &text[1..] {
            ":" => Some(text.to_ascii_uppercase()),
            "%3A" | "%3a" => Some(format!("{}:", text[0..1].to_ascii_uppercase())),
            _ => None,
        }
    }

    if let Some(mut segments) = uri.path_segments() {
        if let Some(mut path) = segments.next().and_then(fix_drive_letter) {
            for segment in segments {
                path.push('/');
                path.push_str(segment);
            }

            uri.set_path(&path);
        }
    }

    uri.set_fragment(None);
}
