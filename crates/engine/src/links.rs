//! ArtCraft community and project links (Help menu, About, start screen, CLI, MCP).

use serde_json::{Value, json};

/// The app's slug on getartcraft.com and GitHub.
pub const APP: &str = "designcraft";
/// The ArtCraft website.
pub const WEBSITE: &str = "https://kojalytics.com";
/// DesignCraft's page on the ArtCraft website.
pub const APP_PAGE: &str = "https://www.kojalytics.com/koja-apps";
/// Where to report bugs and request features.
pub const ISSUES: &str = "https://www.kojalytics.com/contact";

/// All links as JSON (the `app.links` command).
pub fn all() -> Value {
    json!({
        "website": WEBSITE,
        "appPage": APP_PAGE,
        "issues": ISSUES,
    })
}

/// A link by key (`website`, `appPage`, `issues`).
pub fn get(key: &str) -> Option<&'static str> {
    Some(match key {
        "website" => WEBSITE,
        "appPage" => APP_PAGE,
        "issues" => ISSUES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_app_slug() {
        assert!(APP_PAGE.ends_with(&format!("/apps/{APP}")));
        for k in ["website", "appPage", "issues"] {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
    }
}
