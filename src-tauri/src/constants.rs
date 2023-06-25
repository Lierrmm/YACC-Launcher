// This file stores various global constants values
use const_format::concatcp;
use std::time::Duration;

// YACC user agent for web requests
pub const APP_USER_AGENT: &str = concatcp!("Yacc/", env!("CARGO_PKG_VERSION"));

// URL of the YACC masterserver
pub const MASTER_SERVER_URL: &str = "https://master.yacc.app";

// server list endpoint
pub const SERVER_BROWSER_ENDPOINT: &str = "/client/servers";

// COD4 Steam App ID
pub const COD4_STEAM_ID: &str = "7940";

// Order in which the sections for release notes should be displayed
pub const SECTION_ORDER: [&str; 10] = [
    "feat", "fix", "docs", "style", "refactor", "build", "test", "i18n", "chore", "other",
];

// GitHub API endpoints for launcher PRs
pub const PULLS_API_ENDPOINT_LAUNCHER: &str =
    "https://api.github.com/repos/Lierrmm/YACC-Launcher/pulls";

// Statistics (players and servers counts) refresh delay
pub const REFRESH_DELAY: Duration = Duration::from_secs(5 * 60);

// YACC-Launcher repo name and org name on GitHub
pub const YACC_LAUNCHER_REPO_NAME: &str = "Lierrmm/YACC-Launcher";

// YACC release repo name and org name on GitHub
pub const YACC_RELEASE_REPO_NAME: &str = "Lierrmm/yacc";

// URL to launcher commits API URL
pub const YACC_LAUNCHER_COMMITS_API_URL: &str =
    "https://api.github.com/repos/Lierrmm/YACC-Launcher/commits";
