use crate::vendors::amazon::AwsSynthesizer;
use crate::vendors::cartesia::CartesiaSynthesizer;
use crate::vendors::elevenlabs::ElevenlabsSynthesizer;
use crate::vendors::google::{GoogleGender, GoogleSynthesizer, GoogleTTSLanguage, GoogleVoice};
use crate::vendors::playht::PlayhtSynthesizer;
use crate::vendors::rimelabs::RimelabsSynthesizer;
use crate::vendors::verbio::VerbioSynthesizer;
use crate::vendors::whisper::WhisperSynthesizer;
use serde::{Deserialize, Serialize};

//todo move this to an enum where we can have a subset on languages, genders and voices
// In-progress

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Synthesizer {
    pub vendor: SynthesizerVendor,

    pub label: Option<String>,

    #[serde(flatten)]
    pub synthesizer_options: Option<SynthesizerOptions>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SynthesizerOptions {
    Google(GoogleSynthesizer),
    Aws(AwsSynthesizer),
    Cartesia(CartesiaSynthesizer),
    Elevenlabs(ElevenlabsSynthesizer),
    Playht(PlayhtSynthesizer),
    Rimelabs(RimelabsSynthesizer),
    Verbio(VerbioSynthesizer),
    Whisper(WhisperSynthesizer),
}

impl Synthesizer {
    pub fn google() -> Synthesizer {
        Synthesizer {
            vendor: SynthesizerVendor::Google,
            label: None,
            synthesizer_options: Some(SynthesizerOptions::Google(GoogleSynthesizer {
                language: GoogleTTSLanguage::EnglishUnitedKingdom,
                voice: Some(GoogleVoice::EnglishUkStandardAFemale),
                gender: Some(GoogleGender::Male),
            })),
        }
    }

    pub fn amazon() -> Synthesizer {
        Synthesizer {
            vendor: SynthesizerVendor::Aws,
            label: None,
            synthesizer_options: Some(SynthesizerOptions::Aws(AwsSynthesizer {
                  //todo
                })),
        }
    }

    pub fn cartesia() -> Synthesizer {
        Synthesizer {
            vendor: SynthesizerVendor::Cartesia,
            label: None,
            synthesizer_options: Some(SynthesizerOptions::Cartesia(CartesiaSynthesizer {
                //todo
            })),
        }
    }

    pub fn elevenlabs() -> Synthesizer {
        Synthesizer {
            vendor: SynthesizerVendor::Elevenlabs,
            label: None,
            synthesizer_options: Some(SynthesizerOptions::Elevenlabs(ElevenlabsSynthesizer {
                //todo
            })),
        }
    }

    pub fn playht() -> Synthesizer {
        Synthesizer {
            vendor: SynthesizerVendor::Playht,
            label: None,
            synthesizer_options: Some(SynthesizerOptions::Playht(PlayhtSynthesizer {
                //todo
            })),
        }
    }

    pub fn rimelabs() -> Synthesizer {
        Synthesizer {
            vendor: SynthesizerVendor::Rimelabs,
            label: None,
            synthesizer_options: Some(SynthesizerOptions::Rimelabs(RimelabsSynthesizer {
                //todo
            })),
        }
    }

    pub fn verbio() -> Synthesizer {
        Synthesizer {
            vendor: SynthesizerVendor::Verbio,
            label: None,
            synthesizer_options: Some(SynthesizerOptions::Verbio(VerbioSynthesizer {
                //todo
            })),
        }
    }

    pub fn whisper() -> Synthesizer {
        Synthesizer {
            vendor: SynthesizerVendor::Whisper,
            label: None,
            synthesizer_options: Some(SynthesizerOptions::Whisper(WhisperSynthesizer {
                //todo
            })),
        }
    }

    pub fn label(&mut self, label: Option<String>) -> &mut Synthesizer {
        self.label = label;
        self
    }
}

impl Default for Synthesizer {
    fn default() -> Synthesizer {
        Synthesizer {
            vendor: SynthesizerVendor::Default,
            label: None,
            synthesizer_options: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum SynthesizerVendor {
    Default,
    Aws,
    Google,
    Cartesia,
    Elevenlabs,
    Playht,
    Rimelabs,
    Verbio,
    Whisper,
}
