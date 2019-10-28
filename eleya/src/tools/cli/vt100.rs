pub mod escapes {
    pub const BOLD: &str        = "\x1B[1m";
    pub const DIM: &str         = "\x1B[2m";
    pub const UNDERLINED: &str  = "\x1B[4m";
    pub const BLINK: &str       = "\x1B[5m";
    pub const REVERSE: &str     = "\x1B[7m";
    pub const HIDDEN: &str      = "\x1B[8m";


    pub const RESET: &str       = "\x1B[0m";

    pub const DEFAULT_FG: &str  = "\x1B[39m";
    pub const BLACK: &str       = "\x1B[30m";
    pub const RED: &str         = "\x1B[31m";
    pub const GREEN: &str       = "\x1B[32m";
    pub const YELLOW: &str      = "\x1B[33m";
    pub const BLUE: &str        = "\x1B[34m";
    pub const MAGENTA: &str     = "\x1B[35m";
    pub const CYAN: &str        = "\x1B[36m";
    pub const LIGHT_GRAY: &str  = "\x1B[37m";

    pub const DARK_GRAY: &str      = "\x1B[90m";
    pub const LIGHT_RED: &str      = "\x1B[91m";
    pub const LIGHT_GREEN: &str    = "\x1B[92m";
    pub const LIGHT_YELLOW: &str   = "\x1B[93m";
    pub const LIGHT_BLUE: &str     = "\x1B[94m";
    pub const LIGHT_MAGENTA: &str  = "\x1B[95m";
    pub const LIGHT_CYAN: &str     = "\x1B[96m";
    pub const WHITE: &str          = "\x1B[97m";
}