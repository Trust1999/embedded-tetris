use serde::{Deserialize, Serialize};
use esp_idf_svc::nvs::{EspNvs, NvsDefault};

const MAX_HIGHSCORES: usize = 10;
pub const NVS_NAMESPACE: &str = "highscores";
const NVS_KEY: &str = "scores_v2";

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Highscores {
    pub scores: Vec<u32>,
}

impl Highscores {
    pub fn new() -> Self {
        Highscores {
            scores: Vec::with_capacity(MAX_HIGHSCORES),
        }
    }

    pub fn add_score(&mut self, new_score: u32) {
        self.scores.push(new_score);
        //to sort the highscores
        self.scores.sort_by(|a, b| b.cmp(a));
        self.scores.truncate(MAX_HIGHSCORES);
    }
}

pub fn save_highscores(nvs: &mut EspNvs<NvsDefault>, highscores: &Highscores) -> Result<(), anyhow::Error> {
    // Serialisiert die Highscores-Struktur (jetzt nur eine Liste von Zahlen) in einen JSON-String
    let serialized_scores = serde_json::to_string(highscores)?;
    nvs.set_str(NVS_KEY, &serialized_scores)?;
    Ok(())
}

pub fn load_highscores(nvs: &mut EspNvs<NvsDefault>) -> Result<Highscores, anyhow::Error> {
    // Liest den JSON-String und versucht, ihn zu deserialisieren
    if let Some(serialized_scores) =  nvs.get_str(NVS_KEY, &mut [0u8; 255])? {
        let highscores: Highscores = serde_json::from_str(&serialized_scores)?;
        Ok(highscores)
    } else {
        Ok(Highscores::new())
    }
}