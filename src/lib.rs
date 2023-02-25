use wordnet::{self, Database};
use std::{io::Result, path::Path, collections::HashSet};

pub struct PartsOfSpeech {
    wn: Database
}

// wordnet::PartOfSpeech doesn't derive Eq or Copy, so I'll just reimplement it.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PartOfSpeech {
    Noun,
    Adjective,
    AdjectiveSatellite,
    Verb,
    Adverb,
} 

impl PartsOfSpeech {
    pub fn new(path: Option<&Path>) -> Result<PartsOfSpeech> {
        let wn_path = if let Some(p) = path {
            p
        } else {
            // On Debian, the database directory is located at
            // /usr/share/wordnet and can be installed from the package wordnet-base.
            Path::new("/usr/share/wordnet")
        };
        let wn = Database::open(&wn_path)?;
        Ok(PartsOfSpeech { wn })
    }

    
    pub fn parts_of_speech(&self, word: &str) -> HashSet<PartOfSpeech> {
        self
            .wn
            .senses(word)
            .iter()
            .map(|sense| match sense.part_of_speech {
                wordnet::PartOfSpeech::Noun => PartOfSpeech::Noun,
                wordnet::PartOfSpeech::Adjective => PartOfSpeech::Adjective,
                wordnet::PartOfSpeech::AdjectiveSatellite => PartOfSpeech::AdjectiveSatellite,
                wordnet::PartOfSpeech::Verb => PartOfSpeech::Verb,
                wordnet::PartOfSpeech::Adverb => PartOfSpeech::Adverb,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let pos = PartsOfSpeech::new(None).expect("expect /usr/share/wordnet to exist");
        assert_eq!(pos.parts_of_speech("ball"), HashSet::from([PartOfSpeech::Noun, PartOfSpeech::Verb]));
        assert_eq!(pos.parts_of_speech("fast"), HashSet::from([PartOfSpeech::Noun, PartOfSpeech::Verb, PartOfSpeech::Adjective, PartOfSpeech::AdjectiveSatellite, PartOfSpeech::Adverb]));
    }
}
