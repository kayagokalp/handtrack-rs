use crate::tensor::model::Model;
use anyhow::Result;

#[derive(Debug)]
/// Represents a tensorflow session.
pub struct Session {
    session: tensorflow::Session,
}

impl<'a> Session {
    /// Create a `Session` from given `tensorflow::Session`.
    pub fn new(session: tensorflow::Session) -> Session {
        Session { session }
    }

    /// Returns a reference to the `tensorflow::Session` of a `Session`.
    pub fn session(&'a self) -> &'a tensorflow::Session {
        &self.session
    }

    /// Creates a new session for the given model.
    pub fn from_model(model: &Model) -> Result<Self> {
        let session_opts = tensorflow::SessionOptions::default();
        let graph = model.graph();
        let tf_session = tensorflow::Session::new(&session_opts, graph)?;
        let session = Session::new(tf_session);
        Ok(session)
    }
}
