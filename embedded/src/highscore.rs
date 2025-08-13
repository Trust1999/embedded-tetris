use esp_idf_svc::nvs::{EspNvs, NvsDefault};
use std::num::ParseIntError;

const MAX_HIGHSCORES: usize = 10;
pub const NVS_NAMESPACE: &str = "highscores";
const NVS_KEY: &str = "scores_v2";

#[derive(Debug)]
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

    fn serialize(&self) -> String {
        self.scores
            .iter()
            .map(u32::to_string)
            .reduce(|accum, elem| accum + "," + &elem)
            .unwrap_or_default()
    }

    fn deserialize(string: &str) -> Result<Self, ParseIntError> {
        Ok(Self {
            scores: string.split(",").map(|str| dbg!(str).parse()).fold(
                Ok(Vec::new()),
                |maybe_accum, maybe_elem| {
                    let mut accum = maybe_accum?;
                    accum.push(maybe_elem?);
                    Ok(accum)
                },
            )?,
        })
    }
}

pub fn save_highscores(
    nvs: &mut EspNvs<NvsDefault>,
    highscores: &Highscores,
) -> Result<(), Box<dyn std::error::Error>> {
    nvs.set_str(NVS_KEY, &highscores.serialize())?;
    Ok(())
}

pub fn load_highscores(
    nvs: &mut EspNvs<NvsDefault>,
) -> Result<Highscores, Box<dyn std::error::Error>> {
    // Reads the JSON string and attempts to deserialize it
    if let Some(serialized_scores) = nvs.get_str(NVS_KEY, &mut [0u8; 255])? {
        let highscores: Highscores = Highscores::deserialize(&serialized_scores)?;
        Ok(highscores)
    } else {
        Ok(Highscores::new())
    }
}
