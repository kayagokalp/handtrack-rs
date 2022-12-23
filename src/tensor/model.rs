use anyhow::Result;
use tensorflow::{Graph, ImportGraphDefOptions};

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
        // Load frozen model graph from disk.
        let model_bytes = include_bytes!("assets/frozen_inference_graph.pb");
        let graph_options = ImportGraphDefOptions::new();
        // Create the graph and import the bytes from memory.
        let mut graph = Graph::new();
        graph.import_graph_def(model_bytes, &graph_options)?;
        Ok(Model::new(Box::new(graph)))
    }

    /// Returns a reference to the graph of a `Model`.
    pub fn graph(&'a self) -> &'a Graph {
        &self.graph
    }
}
