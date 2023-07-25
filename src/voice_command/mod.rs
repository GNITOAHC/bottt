pub mod utils;
use utils::vc_handle;

pub use utils::{join, leave};

pub mod play;
pub use play::{pause, play, resume, skip, stop};

pub mod tts;
pub use tts::speak;

use crate::Data;
use poise::Command;
use std::error::Error;
pub fn get_voice_commands() -> Vec<Command<Data, Box<dyn Error + Send + Sync>>> {
    vec![
        join(),
        leave(),
        pause(),
        play(),
        resume(),
        skip(),
        stop(),
        speak(),
    ]
}
