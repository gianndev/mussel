// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

use std::fmt::Display;
use std::ops::Range;
use std::path::{Path, PathBuf};
use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term;
use codespan_reporting::term::termcolor::{StandardStream};
use crate::lexer::{TokenRecord};


pub struct FileSet {
    files: SimpleFiles<FilePath, String>
}

impl FileSet {
    pub fn new() -> Self {
        FileSet {
            files: SimpleFiles::new(),
        }
    }

    pub fn add_file<P: AsRef<Path>>(&mut self, path: P, content: String) -> FileIdentifier {
        let id = self.files.add(FilePath { path: path.as_ref().to_path_buf() }, content);
        FileIdentifier(id)
    }

    pub fn get_content(&self, id: FileIdentifier) -> Option<&str> {
        self.files.get(id.0).ok().map(|r| r.source().as_ref())
    }

    pub fn get_path(&self, id: FileIdentifier) -> Option<&FilePath> {
        self.files.get(id.0).ok().map(|r| r.name())
    }

}

#[derive(Clone, Copy, Debug)]
pub struct FileIdentifier(usize);

#[derive(Clone)]
pub struct FilePath {
    path: PathBuf,
}

impl Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "file:///{}", self.path.to_string_lossy().replace("\\", "/"))
    }
}

pub struct Reporter<'a> {
    files: &'a FileSet,
    config: term::Config,
    writer: StandardStream,
}

impl Reporter<'_> {
    pub fn new(files: &FileSet) -> Reporter {
        let config = term::Config::default();
        Reporter {
            files,
            config,
            writer: StandardStream::stderr(term::termcolor::ColorChoice::Always),
        }
    }

    pub fn report<T : LError>(self, error: T) {
        let diagnostics = error.report();
        for diagnostic in diagnostics {
            term::emit(
                &mut self.writer.lock(),
                &self.config,
                &self.files.files,
                &diagnostic,
            ).expect("Error emitting diagnostic");
        }

    }
}

pub trait LError {

    fn report(&self) -> Vec<Diagnostic<usize>>;

}

fn label(file: FileIdentifier, range: Range<usize>) -> Label<usize> {
    Label::primary(file.0, range)
}
fn secondary_label(file: FileIdentifier, range: Range<usize>) -> Label<usize> {
    Label::secondary(file.0, range)
}

impl LError for Box<dyn LError> {
    fn report(&self) -> Vec<Diagnostic<usize>> {
        (**self).report()
    }
}

pub struct ErrorCollection {
    errors: Vec<Box<dyn LError>>,
}

impl ErrorCollection {
    pub fn new() -> Self {
        ErrorCollection {
            errors: Vec::new(),
        }
    }

    pub fn add_error<T: LError + 'static>(&mut self, error: T) {
        self.errors.push(Box::new(error));
    }
}

impl LError for ErrorCollection {
    fn report(&self) -> Vec<Diagnostic<usize>> {
        let mut diagnostics = Vec::new();
        for error in &self.errors {
            diagnostics.extend(error.report());
        }
        diagnostics
    }
}

pub struct TokenError {
    file: FileIdentifier,
    index: usize
}
impl TokenError {
    pub fn new(file: FileIdentifier, index: usize) -> Self {
        TokenError { file, index }
    }
}

impl LError for TokenError {
    fn report(&self) -> Vec<Diagnostic<usize>> {
        let diagnostic = Diagnostic::error().with_message("Unknown symbol");
        vec![
            diagnostic.with_labels(vec![
                label(self.file, self.index..self.index + 1),
            ])
        ]
    }
}


pub struct UnexpectedTokenError {
    file: FileIdentifier,
    record: TokenRecord,
    message: String,
}


impl UnexpectedTokenError {
    pub fn new(file: FileIdentifier, record: TokenRecord, message: String) -> Self {
        UnexpectedTokenError {
            file,
            record,
            message,
        }
    }
}

impl LError for UnexpectedTokenError {
    fn report(&self) -> Vec<Diagnostic<usize>> {
        let diagnostic = Diagnostic::error()
            .with_message(self.message.clone())
            .with_labels(vec![
                label(self.file, self.record.range()),
            ]);
        vec![diagnostic]
    }
}

pub struct UnexpectedEndOfFileError {
    file: FileIdentifier,
    index: usize,
}
impl UnexpectedEndOfFileError {
    pub fn new(file: FileIdentifier, index: usize) -> Self {
        UnexpectedEndOfFileError {
            file,
            index,
        }
    }
}

impl LError for UnexpectedEndOfFileError {
    fn report(&self) -> Vec<Diagnostic<usize>> {
        let diagnostic = Diagnostic::error()
            .with_message("Unexpected end of file")
            .with_labels(vec![
                label(self.file, self.index-1..self.index),
            ]);
        vec![diagnostic]
    }
}


