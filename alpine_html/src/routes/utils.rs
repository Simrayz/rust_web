use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use rust_html::Template;

use crate::main_layout;

pub const ROUTER_CONTENT: &str = "router-content";
pub const ROUTER_CONTENT_ID: &str = "#router-content";
pub const PAGE_NAV: &str = "page-nav";

pub struct RouteOptions {
    pub fragment: Template,
    pub target: &'static str,
}

impl Default for RouteOptions {
    /*************  ✨ Codeium Command ⭐  *************/
    /// Create a new `RouteOptions` with empty defaults.
    ///
    /// - `fragment` is an empty string.
    /// - `target` is an empty string.
    /******  579c58f6-6fe6-4bc9-ba42-6eec3a783684  *******/
    fn default() -> Self {
        Self {
            fragment: "".into(),
            target: "",
        }
    }
}

pub fn router_fragment_stack(hx_req: HxReq, options: RouteOptions) -> Template {
    if hx_req.is_targeting(ROUTER_CONTENT) {
        options.fragment
    } else {
        main_layout(options.fragment, options.target)
    }
}

/// A struct for the htmx request which can be used
/// as an extractor in axum handle functions.
///
/// ```rust
/// async fn handler(hx_req: HxReq) -> Response {
///     if hx_req.is_targeting("special-id") {
///         fragment
///     } else {
///         markup::body(fragment)
///     }
/// }
/// ```
pub(crate) struct HxReq {
    pub(crate) request: Option<String>,
    pub(crate) target: Option<String>,
}

#[async_trait]
impl<S> FromRequestParts<S> for HxReq
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(Self {
            request: parts
                .headers
                .get("hx-request")
                .map(|x| x.to_str().unwrap().to_string()),
            target: parts
                .headers
                .get("hx-target")
                .map(|x| x.to_str().unwrap().to_string()),
        })
    }
}

impl HxReq {
    pub(crate) fn has_request(&self) -> bool {
        self.request.is_some()
    }

    /// We can only send a fragment back when the
    /// original request was targeting a specific id.
    pub(crate) fn is_targeting<T: AsRef<str>>(&self, target: T) -> bool {
        self.has_request() && self.target.as_ref().is_some_and(|t| t == target.as_ref())
    }
}
