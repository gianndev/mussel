// Copyright (c) 2025 Francesco Giannice
// Licensed under the Apache License, Version 2.0 (http://www.apache.org/licenses/LICENSE-2.0)

use std::fmt::Display;
use std::ops::Range;
use std::path::{Path, PathBuf};
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFiles,
    term,
};
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};

/// Represents a set of files and their content.
/// Only one of these should exist at a time.
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
        let id = self.files.add(FilePath::new(path), content);
        FileIdentifier(id)
    }

    pub fn get_content(&self, id: FileIdentifier) -> Option<&str> {
        self.files.get(id.0).ok().map(|r| r.source().as_ref())
    }

    pub fn get_path(&self, id: FileIdentifier) -> Option<&FilePath> {
        self.files.get(id.0).ok().map(|r| r.name())
    }

}

/// File identifier used to lookup files in the `FileSet`.
/// Every file identifier points to a valid file.
#[derive(Clone, Copy, Debug)]
pub struct FileIdentifier(usize);

/// Wrapper for a file path.
#[derive(Clone)]
pub struct FilePath {
    path: PathBuf,
}

impl FilePath {
    fn new<P: AsRef<Path>>(path: P) -> FilePath {
        FilePath {
            path: path.as_ref().to_path_buf(),
        }
    }
}

impl AsRef<Path> for FilePath {
    fn as_ref(&self) -> &Path {
        &self.path
    }
}

impl Display for FilePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "file:///{}", self.path.to_string_lossy().replace("\\", "/"))
    }
}

/// Error reporter to print errors to stderr.
///
/// Usage:
/// ```
/// let files = FileSet::new();
/// let file_id = files.add_file("example.mus", "let x = 42;".to_string());
///
/// let reporter = Reporter::new(files);
/// reporter.report(error); //reports to stderr
/// ```
pub struct Reporter {
    files: FileSet,
    config: term::Config,

    /// Writer to output the errors to (stderr).
    writer: StandardStream,
}

impl Reporter {
    pub fn new(files: FileSet) -> Reporter {
        let config = term::Config::default();
        Reporter {
            files,
            config,
            writer: StandardStream::stderr(ColorChoice::Always),
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

/// Base trait for all errors.
pub trait LError {
    fn report(&self) -> Vec<Diagnostic<usize>>;
}


/// Creates a code label for a specific source location.
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

/// Used to aggregate multiple errors into a single error.
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


pub struct FileError {
    path: FilePath,
    message: String,
}

impl FileError {
    pub fn new<P: AsRef<Path>>(path: P, message: String) -> Self {
        FileError {
            path: FilePath::new(path),
            message
        }
    }
}

impl LError for FileError {
    fn report(&self) -> Vec<Diagnostic<usize>> {
        let diagnostic = Diagnostic::error()
            .with_message(self.message.clone())
            .with_notes(vec![format!("File: {}", self.path)]);
        vec![diagnostic]
    }
}