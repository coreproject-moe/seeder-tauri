use const_format::concatcp;

const DEBUG: bool = false;

pub const DEBUG_URL: &str = "https://127.0.0.1:9000";
pub const RELEASE_URL: &str = "";

const URL: &str = if DEBUG { DEBUG_URL } else { RELEASE_URL };

pub const LOGIN_ENDPOINT: &str = concatcp!(URL, "gfda");
