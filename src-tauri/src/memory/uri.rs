use crate::error::AppError;

/// Default domain when none is specified.
pub const DEFAULT_DOMAIN: &str = "core";

/// Domains that are allowed for memory creation.
pub const VALID_DOMAINS: &[&str] = &["core", "writer", "game", "notes", "system"];

/// A parsed URI with domain and path components.
#[derive(Debug, Clone)]
pub struct ParsedUri {
    pub domain: String,
    pub path: String,
}

impl ParsedUri {
    /// Format back to `domain://path` string.
    #[allow(dead_code)]
    pub fn to_uri(&self) -> String {
        format!("{}://{}", self.domain, self.path)
    }
}

/// Parse a `domain://path` URI string into its components.
///
/// # Arguments
///
/// * `uri` - A URI like `"core://agent/identity"` or `"system://boot"`.
///
/// # Returns
///
/// A `ParsedUri` with domain and path.
///
/// # Errors
///
/// Returns `AppError::Internal` if the URI format is invalid.
pub fn parse_uri(uri: &str) -> Result<ParsedUri, AppError> {
    let uri = uri.trim();
    if let Some(rest) = uri.strip_suffix("://") {
        return Ok(ParsedUri {
            domain: rest.to_string(),
            path: String::new(),
        });
    }
    if let Some(idx) = uri.find("://") {
        let domain = &uri[..idx];
        let path = &uri[idx + 3..];
        if domain.is_empty() {
            return Err(AppError::Internal("Empty domain in URI".to_string()));
        }
        Ok(ParsedUri {
            domain: domain.to_string(),
            path: path.to_string(),
        })
    } else {
        Ok(ParsedUri {
            domain: DEFAULT_DOMAIN.to_string(),
            path: uri.to_string(),
        })
    }
}

/// Extract the parent path and leaf segment from a path string.
///
/// # Returns
///
/// `("parent/path", "leaf")` or `("", "leaf")` for top-level paths.
pub fn split_parent_path(path: &str) -> (&str, &str) {
    match path.rfind('/') {
        Some(idx) => (&path[..idx], &path[idx + 1..]),
        None => ("", path),
    }
}

/// Check whether a domain is valid for memory creation.
pub fn is_valid_domain(domain: &str) -> bool {
    VALID_DOMAINS.contains(&domain)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_standard_uri() {
        let p = parse_uri("core://agent/identity").unwrap();
        assert_eq!(p.domain, "core");
        assert_eq!(p.path, "agent/identity");
    }

    #[test]
    fn test_parse_domain_only() {
        let p = parse_uri("core://").unwrap();
        assert_eq!(p.domain, "core");
        assert_eq!(p.path, "");
    }

    #[test]
    fn test_parse_no_domain() {
        let p = parse_uri("agent").unwrap();
        assert_eq!(p.domain, "core");
        assert_eq!(p.path, "agent");
    }

    #[test]
    fn test_split_parent_path() {
        assert_eq!(split_parent_path("agent/identity"), ("agent", "identity"));
        assert_eq!(split_parent_path("agent"), ("", "agent"));
    }
}
