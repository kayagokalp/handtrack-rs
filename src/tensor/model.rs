use anyhow::Result;
use std::{fs::File, io::Read};
use tensorflow::{Graph, ImportGraphDefOptions};

use crate::utils::download::{download_frozen_graph, frozen_graph_path};

#[derive(Debug)]
/// Represents a tensorflow model hosted in memory.
pub struct Model {
    graph: Box<Graph>,
}

impl<'a> Model {
    /// Construct a `Model` from given `tensorflow::Graph`.
    pub fn new(graph: Box<Graph>) -> Model {
        Model { graph }
    }

    /// Read the default `frozen_inference_graph.pb` and construct a `Model` from it.
    pub fn from_frozen_graph() -> Result<Model> {
        let model_bytes = frozen_graph()?;
        let graph_options = ImportGraphDefOptions::new();
        // Create the graph and import the bytes from memory.
        let mut graph = Graph::new();
        graph.import_graph_def(&model_bytes, &graph_options)?;
        Ok(Model::new(Box::new(graph)))
    }

    /// Read the graph from provided `Path` and construct a `Model` from it.
    pub fn from_path(
        graph_path: &std::path::Path,
        options: ImportGraphDefOptions,
    ) -> Result<Model> {
        let mut model_bytes = Vec::new();
        let mut graph_file = File::open(graph_path)?;
        graph_file.read_to_end(&mut model_bytes)?;
        let mut graph = Graph::new();
        graph.import_graph_def(&model_bytes, &options)?;
        Ok(Model::new(Box::new(graph)))
    }

    /// Returns a reference to the graph of a `Model`.
    pub fn graph(&'a self) -> &'a Graph {
        &self.graph
    }
}

/// Reads the frozen inference graph from disk.
///
/// If the graph is not downloaded, this downloads the model first.
pub(crate) fn frozen_graph() -> Result<Vec<u8>> {
    // Check if the frozen graph already downloaded.
    let graph_path = frozen_graph_path()?;
    if !graph_path.exists() {
        // Download the the frozen_graph.
        download_frozen_graph()?;
    }
    let mut buffer = Vec::new();
    let mut graph_file = File::open(graph_path)?;
    graph_file.read_to_end(&mut buffer)?;
    Ok(buffer)
}
