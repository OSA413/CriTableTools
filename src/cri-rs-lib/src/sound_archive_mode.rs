use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum CriSoundArchiveFormat {
    Adx = 0,
    Dsp = 4,
    Wav = 5,
}
