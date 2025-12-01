//! URL-based routing using hash URLs.
//!
//! Supports routes like:
//! - `#/puzzle/fill-a-box` - Load a specific puzzle
//! - `#/tutorial/arithmetic` - Load a specific tutorial
//! - (empty or `#/`) - Default sandbox mode

/// Application route parsed from URL hash.
#[derive(Clone, Debug, PartialEq)]
pub enum Route {
    /// Default sandbox mode (no puzzle/tutorial loaded).
    Sandbox,
    /// A puzzle by ID.
    Puzzle(String),
    /// A tutorial by ID (future).
    Tutorial(String),
}

impl Route {
    /// Parse route from URL hash string.
    pub fn from_hash(hash: &str) -> Self {
        let path = hash.trim_start_matches('#').trim_start_matches('/');
        let parts: Vec<&str> = path.split('/').collect();

        match parts.as_slice() {
            ["puzzle", id] if !id.is_empty() => Route::Puzzle((*id).to_string()),
            ["tutorial", id] if !id.is_empty() => Route::Tutorial((*id).to_string()),
            _ => Route::Sandbox,
        }
    }

    /// Convert route to URL hash string.
    pub fn to_hash(&self) -> String {
        match self {
            Route::Sandbox => String::new(),
            Route::Puzzle(id) => format!("#/puzzle/{id}"),
            Route::Tutorial(id) => format!("#/tutorial/{id}"),
        }
    }
}

/// Get the current route from window.location.hash.
pub fn current_route() -> Route {
    let hash = web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_default();
    Route::from_hash(&hash)
}

/// Set the URL hash without triggering a page reload.
pub fn set_route(route: &Route) {
    if let Some(window) = web_sys::window() {
        let hash = route.to_hash();
        let _ = window.location().set_hash(&hash);
    }
}

/// Get puzzle ID from route if it's a puzzle route.
pub fn puzzle_id_from_route(route: &Route) -> Option<&str> {
    match route {
        Route::Puzzle(id) => Some(id),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty() {
        assert_eq!(Route::from_hash(""), Route::Sandbox);
        assert_eq!(Route::from_hash("#"), Route::Sandbox);
        assert_eq!(Route::from_hash("#/"), Route::Sandbox);
    }

    #[test]
    fn test_parse_puzzle() {
        assert_eq!(
            Route::from_hash("#/puzzle/fill-a-box"),
            Route::Puzzle("fill-a-box".to_string())
        );
        assert_eq!(
            Route::from_hash("#/puzzle/make-four"),
            Route::Puzzle("make-four".to_string())
        );
    }

    #[test]
    fn test_parse_tutorial() {
        assert_eq!(
            Route::from_hash("#/tutorial/arithmetic"),
            Route::Tutorial("arithmetic".to_string())
        );
    }

    #[test]
    fn test_to_hash() {
        assert_eq!(Route::Sandbox.to_hash(), "");
        assert_eq!(
            Route::Puzzle("fill-a-box".to_string()).to_hash(),
            "#/puzzle/fill-a-box"
        );
        assert_eq!(
            Route::Tutorial("arithmetic".to_string()).to_hash(),
            "#/tutorial/arithmetic"
        );
    }
}
