//! SigmaOS Coreutils
//!
//! Essential command-line utilities for SigmaOS inspired by GNU Coreutils and BSD Core utilities.

pub mod cat;
pub mod chmod;
pub mod chown;
pub mod cp;
pub mod df;
pub mod du;
pub mod echo;
pub mod env;
pub mod false_module;
pub mod head;
pub mod ls;
pub mod mkdir;
pub mod mv;
pub mod pwd;
pub mod rm;
pub mod tail;
pub mod tee;
pub mod touch;
pub mod true_module;
pub mod uname;
pub mod uniq;
pub mod wc;

pub use cat::run as cat_run;
pub use chmod::run as chmod_run;
pub use chown::run as chown_run;
pub use cp::run as cp_run;
pub use df::run as df_run;
pub use du::run as du_run;
pub use echo::run as echo_run;
pub use env::run as env_run;
pub use false_module::main as false_main;
pub use head::{run as head_run, HeadOptions};
pub use ls::run as ls_run;
pub use mkdir::run as mkdir_run;
pub use mv::run as mv_run;
pub use pwd::run as pwd_run;
pub use rm::run as rm_run;
pub use tail::{run as tail_run, TailOptions};
pub use tee::{run as tee_run, TeeOptions};
pub use touch::run as touch_run;
pub use true_module::main as true_main;
pub use uname::{run as uname_run, SystemInfo, UnameOptions};
pub use uniq::{run as uniq_run, UniqOptions};
pub use wc::{count_reader as wc_count_reader, run as wc_run, WcOptions, WcStats};
