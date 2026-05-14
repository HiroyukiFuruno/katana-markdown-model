mod html;
mod metadata_demo;
mod model;

use html::HtmlRenderer;
use katana_markdown_model::{KatanaMarkdownModel, MarkdownInput};
use metadata_demo::MetadataDemo;
use model::{HarnessDocument, HarnessError};
use std::path::PathBuf;

const DEFAULT_KATANA_SAMPLE: &str =
    "/Users/hiroyuki_furuno/works/private/katana/assets/fixtures/sample.md";
const FALLBACK_FIXTURE: &str = "tests/fixtures/canonical/katana_sample.md";

fn main() -> Result<(), HarnessError> {
    let markdown_path = markdown_path();
    let source = std::fs::read_to_string(&markdown_path)
        .map_err(|source| HarnessError::ReadFixture(markdown_path.clone(), source))?;
    let document = KatanaMarkdownModel::parse(MarkdownInput::from_content(
        markdown_path.to_string_lossy().to_string(),
        source.clone(),
    ))
    .map_err(HarnessError::Parse)?;
    let metadata_result = KatanaMarkdownModel::reconcile(MetadataDemo::request());
    let output = output_path()?;

    HtmlRenderer::write(
        &output,
        &HarnessDocument::new(markdown_path, source, document, metadata_result),
    )?;
    println!("KMM manual harness:");
    println!("{}", output.display());
    Ok(())
}

fn markdown_path() -> PathBuf {
    let explicit = std::env::args().nth(1).map(PathBuf::from);
    if let Some(path) = explicit {
        return path;
    }
    let katana_sample = PathBuf::from(DEFAULT_KATANA_SAMPLE);
    if katana_sample.exists() {
        return katana_sample;
    }
    PathBuf::from(FALLBACK_FIXTURE)
}

fn output_path() -> Result<PathBuf, HarnessError> {
    let directory = std::env::current_dir()
        .map_err(HarnessError::CurrentDirectory)?
        .join("target")
        .join("manual-harness");
    std::fs::create_dir_all(&directory)
        .map_err(|source| HarnessError::CreateDirectory(directory.clone(), source))?;
    Ok(directory.join("index.html"))
}
