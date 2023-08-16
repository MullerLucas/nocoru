use core::fmt;
use std::{result, error, process, sync};




pub type HellResult<T> = result::Result<T, HellError>;
pub type HellErrorRef = sync::Arc<dyn error::Error>;



// ----------------------------------------------------------------------------
// hell error
// ----------------------------------------------------------------------------

/// we need to wrap the acutal error struct into another struct, so we can implement From<E> for any Error (including HellError)
/// using a !not Trait implementation would be nicer
#[derive(Clone, fmt::Debug)]
pub struct HellError {
    #[allow(dead_code)]
    inner: InnerHellError,
}

// TODO: check
unsafe impl Send for HellError  {}

impl HellError {
    pub fn new(kind: HellErrorKind, content: HellErrorContent) -> Self {
        Self {
            inner: InnerHellError { kind, content }
        }
    }

    pub fn from_source<E>(kind: HellErrorKind, source: E) -> Self
    where E: error::Error + 'static
    {
        Self::new(kind, HellErrorContent::Wrapper(sync::Arc::new(source)))
    }

    pub fn from_msg(kind: HellErrorKind, msg: String) -> Self {
        Self::new(kind, HellErrorContent::Message(msg))
    }
}

impl<E> From<E> for HellError where E: error::Error + 'static {
    fn from(err : E) -> HellError {
        HellError::new(
            HellErrorKind::GenericError,
            HellErrorContent::Wrapper(sync::Arc::new(err))
        )
    }
}


// ----------------------------------------------------------------------------
// inner-hell-error-helper
// ----------------------------------------------------------------------------
pub struct HellErrorHelper;

impl HellErrorHelper {
    pub fn render_msg_err(msg: impl Into<String>) -> HellError {
        HellError::new(HellErrorKind::RenderError, HellErrorContent::Message(msg.into()))
    }

    pub fn request_msg_err(msg: impl Into<String>) -> HellError {
        let msg = msg.into();
        eprintln!("[ERR]: {msg}");
        HellError::new(HellErrorKind::RequestError, HellErrorContent::Message(msg))
    }
}



// ----------------------------------------------------------------------------
// inner-hell-error
// ----------------------------------------------------------------------------

#[derive(Clone, fmt::Debug)]
pub enum HellErrorKind  {
    GenericError,
    WindowError,
    RenderError,
    ResourceError,
    WebError,
    RequestError,
}

#[derive(Clone, fmt::Debug)]
pub enum HellErrorContent {
    Empty,
    Wrapper(HellErrorRef),
    Message(String),
    Code(u32)
}

#[derive(Clone, fmt::Debug)]
pub struct InnerHellError {
    kind: HellErrorKind,
    content: HellErrorContent,
}


impl InnerHellError {
    #[allow(dead_code)]
    pub fn new(kind: HellErrorKind, content: HellErrorContent) -> Self {
        Self { kind, content, }
    }

    pub fn kind(&self) -> &HellErrorKind {
        &self.kind
    }

    pub fn content(&self) -> &HellErrorContent {
        &self.content
    }
}

impl fmt::Display for InnerHellError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self.content() {
            HellErrorContent::Empty        => write!(f, "[{:?}] error has no content", self.kind()),
            HellErrorContent::Wrapper(err) => err.fmt(f),
            HellErrorContent::Message(msg) => write!(f, "[{:?}] {}", self.kind(), msg),
            HellErrorContent::Code(code)   => write!(f, "[{:?}] Error-Code: '{}'", self.kind(), code)
        }
    }
}

impl error::Error for InnerHellError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self.content() {
            HellErrorContent::Wrapper(err) => Some(err.as_ref()),
            _ => None
        }
    }

    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

impl process::Termination for InnerHellError {
    fn report(self) -> std::process::ExitCode {
        process::ExitCode::FAILURE
    }
}



// ----------------------------------------------------------------------------
//  err-to-hell-err
// ----------------------------------------------------------------------------

pub trait ErrToHellErr<V, E>
where E: error::Error
{
    fn to_hell_err(self, kind: HellErrorKind) -> Result<V, HellError>;
    fn to_generic_hell_err(self) -> Result<V, HellError>;
    fn to_render_hell_err(self) -> Result<V, HellError>;
}

impl<V, E> ErrToHellErr<V, E> for Result<V, E>
where
    E: error::Error + 'static
{
    fn to_hell_err(self, kind: HellErrorKind) -> Result<V, HellError> {
        self.map_err(|e| {
            HellError::from_source(kind, e)
        })
    }

    fn to_generic_hell_err(self) -> Result<V, HellError> {
        self.map_err(|e| {
            HellError::from_source(HellErrorKind::GenericError, e)
        })
    }

    fn to_render_hell_err(self) -> Result<V, HellError> {
        self.map_err(|e| {
            HellError::from_source(HellErrorKind::RenderError, e)
        })
    }
}



// ----------------------------------------------------------------------------
//  err-to-hell-err
// ----------------------------------------------------------------------------

pub trait OptToHellErr<V> {
    fn to_generic_hell_err(self) -> Result<V, HellError>;
    fn to_window_hell_err(self) -> Result<V, HellError>;
    fn to_render_hell_err(self) -> Result<V, HellError>;
    fn ok_or_herr(self, kind: HellErrorKind) -> HellResult<V>;
    fn ok_or_render_herr(self, msg: impl Into<String>) -> HellResult<V>;
}

impl<V> OptToHellErr<V> for Option<V> {
    fn ok_or_herr(self, kind: HellErrorKind) -> Result<V, HellError> {
        self.ok_or_else(|| HellError::from_msg(kind, "option is none".to_string()))
    }

    fn to_generic_hell_err(self) -> Result<V, HellError> {
        self.ok_or_herr(HellErrorKind::GenericError)
    }

    fn to_window_hell_err(self) -> Result<V, HellError> {
        self.ok_or_herr(HellErrorKind::WindowError)
    }

    fn to_render_hell_err(self) -> Result<V, HellError> {
        self.ok_or_herr(HellErrorKind::RenderError)
    }

    fn ok_or_render_herr(self, msg: impl Into<String>) -> HellResult<V> {
        self.ok_or_else(|| HellError::from_msg(HellErrorKind::RenderError, msg.into()))
    }
}


// -------------------

#[cfg(feature = "axum")]
impl axum::response::IntoResponse for HellError {
    fn into_response(self) -> axum::response::Response {
        use axum::http::StatusCode;
        let body = match self.inner.kind {
            HellErrorKind::GenericError => "Generic Error",
            HellErrorKind::WindowError  => "Window Error",
            HellErrorKind::RenderError  => "Render Error",
            HellErrorKind::ResourceError => "Resource Error",
            HellErrorKind::WebError     => "Web Error",
            HellErrorKind::RequestError => "Request Error",
        };

        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}
