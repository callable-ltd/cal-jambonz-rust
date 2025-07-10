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

impl SynthesizerVendor {
    /// Determines the synthesizer vendor from a TTS voice string
    pub fn from_voice_string(voice: &str) -> Self {
        // Google voices - contain these specific markers
        if voice.contains("-Standard-") ||
            voice.contains("-Wavenet-") ||
            voice.contains("-Neural-") ||
            voice.contains("-Studio-") ||
            voice.contains("-Polyglot-") ||
            voice.contains("-News-") ||
            voice.contains("-Casual-") {
            return SynthesizerVendor::Google;
        }

        // AWS/Microsoft voices - various patterns
        if voice.starts_with("Microsoft:") ||
            voice.starts_with("Azure:") ||
            voice.starts_with("Polly:") ||
            voice.starts_with("aws:") ||
            voice.ends_with("Neural") ||
            voice.ends_with("Multilingual") {
            return SynthesizerVendor::Aws;
        }

        // Check for voice patterns like "en-US-JennyNeural" (AWS/Microsoft style)
        // These have a name starting with uppercase after the last dash
        if let Some(last_dash_pos) = voice.rfind('-') {
            let suffix = &voice[last_dash_pos + 1..];
            if suffix.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                return SynthesizerVendor::Aws;
            }
        }

        // Simple locale patterns like "en-US", "en-GB" default to Google
        if voice.len() == 5 && voice.chars().nth(2) == Some('-') {
            return SynthesizerVendor::Google;
        }

        // Default fallback
        SynthesizerVendor::Default
    }
}

impl SynthesizerOptions {
    /// Creates SynthesizerOptions from a TTS voice string
    pub fn from_voice_string(voice: &str) -> Option<Self> {
        match SynthesizerVendor::from_voice_string(voice) {
            SynthesizerVendor::Google => {
                // Parse the voice string to get language and voice enum
                let language = GoogleTTSLanguage::from_voice_string(voice)?;
                let google_voice = GoogleVoice::from_voice_string(voice)?;

                // Determine gender from the voice identifier (A/C = Female, B/D = Male)
                let gender = match voice {
                    v if v.contains("-A-") || v.contains("-C-") || v.contains("-F-") || v.contains("-O-") => Some(GoogleGender::Female),
                    v if v.contains("-B-") || v.contains("-D-") || v.contains("-E-") => Some(GoogleGender::Male),
                    _ => None,
                };

                Some(SynthesizerOptions::Google(GoogleSynthesizer {
                    language,
                    voice: Some(google_voice),
                    gender,
                }))
            }
            SynthesizerVendor::Aws => {
                // For now, just return empty AWS synthesizer
                Some(SynthesizerOptions::Aws(AwsSynthesizer {}))
            }
            _ => None,
        }
    }
}

impl GoogleVoice {
    /// Parses a voice string like "en-GB-Standard-A" and returns the voice enum
    pub fn from_voice_string(voice: &str) -> Option<Self> {
        match voice {
            // Arabic
            "ar-XA-Standard-A" => Some(GoogleVoice::ArabicStandardAFemale),
            "ar-XA-Standard-B" => Some(GoogleVoice::ArabicStandardBMale),
            "ar-XA-Standard-C" => Some(GoogleVoice::ArabicStandardCMale),
            "ar-XA-Standard-D" => Some(GoogleVoice::ArabicStandardDFemale),
            "ar-XA-Wavenet-A" => Some(GoogleVoice::ArabicWavenetAFemale),
            "ar-XA-Wavenet-B" => Some(GoogleVoice::ArabicWavenetBMale),
            "ar-XA-Wavenet-C" => Some(GoogleVoice::ArabicWavenetCMale),

            // Czech
            "cs-CZ-Standard-A" => Some(GoogleVoice::CzechStandardAFemale),
            "cs-CZ-Wavenet-A" => Some(GoogleVoice::CzechWavenetAFemale),

            // Danish
            "da-DK-Standard-A" => Some(GoogleVoice::DanishStandardAFemale),
            "da-DK-Wavenet-A" => Some(GoogleVoice::DanishWavenetAFemale),

            // Dutch
            "nl-NL-Standard-A" => Some(GoogleVoice::DutchStandardAFemale),
            "nl-NL-Standard-B" => Some(GoogleVoice::DutchStandardBMale),
            "nl-NL-Standard-C" => Some(GoogleVoice::DutchStandardCMale),
            "nl-NL-Standard-D" => Some(GoogleVoice::DutchStandardDFemale),
            "nl-NL-Standard-E" => Some(GoogleVoice::DutchStandardEFemale),
            "nl-NL-Wavenet-A" => Some(GoogleVoice::DutchWavenetAFemale),
            "nl-NL-Wavenet-B" => Some(GoogleVoice::DutchWavenetBMale),
            "nl-NL-Wavenet-C" => Some(GoogleVoice::DutchWavenetCMale),
            "nl-NL-Wavenet-D" => Some(GoogleVoice::DutchWavenetDFemale),
            "nl-NL-Wavenet-E" => Some(GoogleVoice::DutchWavenetEFemale),

            // English - Australia
            "en-AU-Standard-A" => Some(GoogleVoice::EnglishAuStandardAFemale),
            "en-AU-Standard-B" => Some(GoogleVoice::EnglishAuStandardBMale),
            "en-AU-Standard-C" => Some(GoogleVoice::EnglishAuStandardCFemale),
            "en-AU-Standard-D" => Some(GoogleVoice::EnglishAuStandardDMale),
            "en-AU-Wavenet-A" => Some(GoogleVoice::EnglishAuWavenetAFemale),
            "en-AU-Wavenet-B" => Some(GoogleVoice::EnglishAuWavenetBMale),
            "en-AU-Wavenet-C" => Some(GoogleVoice::EnglishAuWavenetCFemale),
            "en-AU-Wavenet-D" => Some(GoogleVoice::EnglishAuWavenetDMale),

            // English - India
            "en-IN-Standard-A" => Some(GoogleVoice::EnglishIndiaStandardAFemale),
            "en-IN-Standard-B" => Some(GoogleVoice::EnglishIndiaStandardBMale),
            "en-IN-Standard-C" => Some(GoogleVoice::EnglishIndiaStandardCMale),
            "en-IN-Standard-D" => Some(GoogleVoice::EnglishIndiaStandardDFemale),
            "en-IN-Wavenet-A" => Some(GoogleVoice::EnglishIndiaWavenetAFemale),
            "en-IN-Wavenet-B" => Some(GoogleVoice::EnglishIndiaWavenetBMale),
            "en-IN-Wavenet-C" => Some(GoogleVoice::EnglishIndiaWavenetCMale),
            "en-IN-Wavenet-D" => Some(GoogleVoice::EnglishIndiaWavenetDFemale),

            // English - UK
            "en-GB-Chirp-HD-D" => Some(GoogleVoice::EnglishUkChirpHdDMale),
            "en-GB-Chirp-HD-F" => Some(GoogleVoice::EnglishUkChirpHdFFemale),
            "en-GB-Chirp-HD-O" => Some(GoogleVoice::EnglishUkChirpHdOFemale),
            "en-GB-News-G" => Some(GoogleVoice::EnglishUkNewsGFemale),
            "en-GB-News-H" => Some(GoogleVoice::EnglishUkNewsHFemale),
            "en-GB-News-I" => Some(GoogleVoice::EnglishUkNewsIFemale),
            "en-GB-News-J" => Some(GoogleVoice::EnglishUkNewsJMale),
            "en-GB-News-K" => Some(GoogleVoice::EnglishUkNewsKMale),
            "en-GB-News-L" => Some(GoogleVoice::EnglishUkNewsLMale),
            "en-GB-News-M" => Some(GoogleVoice::EnglishUkNewsMMale),
            "en-GB-Standard-A" => Some(GoogleVoice::EnglishUkStandardAFemale),
            "en-GB-Standard-B" => Some(GoogleVoice::EnglishUkStandardBMale),
            "en-GB-Standard-C" => Some(GoogleVoice::EnglishUkStandardCFemale),
            "en-GB-Standard-D" => Some(GoogleVoice::EnglishUkStandardDMale),
            "en-GB-Standard-F" => Some(GoogleVoice::EnglishUkStandardFFemale),
            "en-GB-Standard-N" => Some(GoogleVoice::EnglishUkStandardNFemale),
            "en-GB-Standard-O" => Some(GoogleVoice::EnglishUkStandardOMale),
            "en-GB-Studio-B" => Some(GoogleVoice::EnglishUkStudioBMale),
            "en-GB-Studio-C" => Some(GoogleVoice::EnglishUkStudioCFemale),
            "en-GB-Wavenet-A" => Some(GoogleVoice::EnglishUkWavenetAFemale),
            "en-GB-Wavenet-B" => Some(GoogleVoice::EnglishUkWavenetBMale),
            "en-GB-Wavenet-C" => Some(GoogleVoice::EnglishUkWavenetCFemale),
            "en-GB-Wavenet-D" => Some(GoogleVoice::EnglishUkWavenetDMale),
            "en-GB-Wavenet-F" => Some(GoogleVoice::EnglishUkWavenetFFemale),
            "en-GB-Wavenet-N" => Some(GoogleVoice::EnglishUkWavenetOMale),
            "en-GB-Neural2-N" => Some(GoogleVoice::EnglishUkNeural2N),
            "en-GB-Neural2-O" => Some(GoogleVoice::EnglishUkNeural2O),

            // English - US
            "en-US-Standard-B" => Some(GoogleVoice::EnglishUsStandardBMale),
            "en-US-Standard-C" => Some(GoogleVoice::EnglishUsStandardCFemale),
            "en-US-Standard-D" => Some(GoogleVoice::EnglishUsStandardDMale),
            "en-US-Standard-E" => Some(GoogleVoice::EnglishUsStandardEMale),
            "en-US-Wavenet-A" => Some(GoogleVoice::EnglishUsWavenetAMale),
            "en-US-Wavenet-B" => Some(GoogleVoice::EnglishUsWavenetBMale),
            "en-US-Wavenet-C" => Some(GoogleVoice::EnglishUsWavenetCFemale),
            "en-US-Wavenet-D" => Some(GoogleVoice::EnglishUsWavenetDMale),
            "en-US-Wavenet-E" => Some(GoogleVoice::EnglishUsWavenetEFemale),
            "en-US-Wavenet-F" => Some(GoogleVoice::EnglishUsWavenetFFemale),

            // Filipino
            "fil-PH-Standard-A" => Some(GoogleVoice::FilipinoPhStandardAFemale),
            "fil-PH-Wavenet-A" => Some(GoogleVoice::FilipinoPhWavenetAFemale),

            // Finnish
            "fi-FI-Standard-A" => Some(GoogleVoice::FinnishFiStandardAFemale),
            "fi-FI-Wavenet-A" => Some(GoogleVoice::FinnishFiWavenetAFemale),

            // French - Canada
            "fr-CA-Standard-A" => Some(GoogleVoice::FrenchCaStandardAFemale),
            "fr-CA-Standard-B" => Some(GoogleVoice::FrenchCaStandardBMale),
            "fr-CA-Standard-C" => Some(GoogleVoice::FrenchCaStandardCFemale),
            "fr-CA-Standard-D" => Some(GoogleVoice::FrenchCaStandardDMale),
            "fr-CA-Wavenet-A" => Some(GoogleVoice::FrenchCaWavenetAFemale),
            "fr-CA-Wavenet-B" => Some(GoogleVoice::FrenchCaWavenetBMale),
            "fr-CA-Wavenet-C" => Some(GoogleVoice::FrenchCaWavenetCFemale),
            "fr-CA-Wavenet-D" => Some(GoogleVoice::FrenchCaWavenetDMale),

            // French - France
            "fr-FR-Standard-A" => Some(GoogleVoice::FrenchFrStandardAFemale),
            "fr-FR-Standard-B" => Some(GoogleVoice::FrenchFrStandardBMale),
            "fr-FR-Standard-C" => Some(GoogleVoice::FrenchFrStandardCFemale),
            "fr-FR-Standard-D" => Some(GoogleVoice::FrenchFrStandardDMale),
            "fr-FR-Standard-E" => Some(GoogleVoice::FrenchFrStandardEFemale),
            "fr-FR-Wavenet-A" => Some(GoogleVoice::FrenchFrWavenetAFemale),
            "fr-FR-Wavenet-B" => Some(GoogleVoice::FrenchFrWavenetBMale),
            "fr-FR-Wavenet-C" => Some(GoogleVoice::FrenchFrWavenetCFemale),
            "fr-FR-Wavenet-D" => Some(GoogleVoice::FrenchFrWavenetDMale),
            "fr-FR-Wavenet-E" => Some(GoogleVoice::FrenchFrWavenetEFemale),

            // German
            "de-DE-Standard-A" => Some(GoogleVoice::GermanDeStandardAFemale),
            "de-DE-Standard-B" => Some(GoogleVoice::GermanDeStandardBMale),
            "de-DE-Standard-C" => Some(GoogleVoice::GermanDeStandardEMale),  // Note: enum says C but is EMale
            "de-DE-Standard-D" => Some(GoogleVoice::GermanDeStandardFFemale), // Note: enum says D but is FFemale
            "de-DE-Wavenet-A" => Some(GoogleVoice::GermanDeWavenetAFemale),
            "de-DE-Wavenet-B" => Some(GoogleVoice::GermanDeWavenetBMale),
            "de-DE-Wavenet-C" => Some(GoogleVoice::GermanDeWavenetCFemale),
            "de-DE-Wavenet-D" => Some(GoogleVoice::GermanDeWavenetDMale),
            "de-DE-Wavenet-E" => Some(GoogleVoice::GermanDeWavenetEMale),
            "de-DE-Wavenet-F" => Some(GoogleVoice::GermanDeWavenetFFemale),

            // Greek
            "el-GR-Standard-A" => Some(GoogleVoice::GreekGrStandardAFemale),
            "el-GR-Wavenet-A" => Some(GoogleVoice::GreekGrWavenetAFemale),

            // Hindi
            "hi-IN-Standard-A" => Some(GoogleVoice::HindiIndiaStandardAFemale),
            "hi-IN-Standard-B" => Some(GoogleVoice::HindiIndiaStandardBMale),
            "hi-IN-Standard-C" => Some(GoogleVoice::HindiIndiaStandardCMale),
            "hi-IN-Standard-D" => Some(GoogleVoice::HindiIndiaStandardDFemale),
            "hi-IN-Wavenet-A" => Some(GoogleVoice::HindiIndiaWavenetAFemale),
            "hi-IN-Wavenet-B" => Some(GoogleVoice::HindiIndiaWavenetBMale),
            "hi-IN-Wavenet-C" => Some(GoogleVoice::HindiIndiaWavenetCMale),
            "hi-IN-Wavenet-D" => Some(GoogleVoice::HindiIndiaWavenetDFemale),

            // Hungarian
            "hu-HU-Standard-A" => Some(GoogleVoice::HungarianHuStandardAFemale),
            "hu-HU-Wavenet-A" => Some(GoogleVoice::HungarianHuWavenetAFemale),

            // Indonesian
            "id-ID-Standard-A" => Some(GoogleVoice::IndonesianIdStandardAFemale),
            "id-ID-Standard-B" => Some(GoogleVoice::IndonesianIdStandardBMale),
            "id-ID-Standard-C" => Some(GoogleVoice::IndonesianIdStandardCMale),
            "id-ID-Standard-D" => Some(GoogleVoice::IndonesianIdStandardDFemale),
            "id-ID-Wavenet-A" => Some(GoogleVoice::IndonesianIdWavenetAFemale),
            "id-ID-Wavenet-B" => Some(GoogleVoice::IndonesianIdWavenetBMale),
            "id-ID-Wavenet-C" => Some(GoogleVoice::IndonesianIdWavenetCMale),
            "id-ID-Wavenet-D" => Some(GoogleVoice::IndonesianIdWavenetDFemale),

            // Italian
            "it-IT-Standard-A" => Some(GoogleVoice::ItalianItStandardAFemale),
            "it-IT-Standard-B" => Some(GoogleVoice::ItalianItStandardBFemale),
            "it-IT-Standard-C" => Some(GoogleVoice::ItalianItStandardCMale),
            "it-IT-Standard-D" => Some(GoogleVoice::ItalianItStandardDMale),
            "it-IT-Wavenet-A" => Some(GoogleVoice::ItalianItWavenetAFemale),
            "it-IT-Wavenet-B" => Some(GoogleVoice::ItalianItWavenetBFemale),
            "it-IT-Wavenet-C" => Some(GoogleVoice::ItalianItWavenetCMale),
            "it-IT-Wavenet-D" => Some(GoogleVoice::ItalianItWavenetDMale),

            // Japanese (Note: your enum has "js-JP" but it should be "ja-JP")
            "ja-JP-Standard-A" => Some(GoogleVoice::JapaneseJpStandardAFemale),
            "ja-JP-Standard-B" => Some(GoogleVoice::JapaneseJpStandardBFemale),
            "ja-JP-Standard-C" => Some(GoogleVoice::JapaneseJpStandardCMale),
            "ja-JP-Standard-D" => Some(GoogleVoice::JapaneseJpStandardDMale),
            "ja-JP-Wavenet-A" => Some(GoogleVoice::JapaneseJpWavenetAFemale),
            "ja-JP-Wavenet-B" => Some(GoogleVoice::JapaneseJpWavenetBFemale),
            "ja-JP-Wavenet-C" => Some(GoogleVoice::JapaneseJpWavenetCMale),
            "ja-JP-Wavenet-D" => Some(GoogleVoice::JapaneseJpWavenetDMale),

            // Korean
            "ko-KR-Standard-A" => Some(GoogleVoice::KoreanKrStandardAFemale),
            "ko-KR-Standard-B" => Some(GoogleVoice::KoreanKrStandardBFemale),
            "ko-KR-Standard-C" => Some(GoogleVoice::KoreanKrStandardCMale),
            "ko-KR-Standard-D" => Some(GoogleVoice::KoreanKrStandardDMale),
            "ko-KR-Wavenet-A" => Some(GoogleVoice::KoreanKrWavenetAFemale),
            "ko-KR-Wavenet-B" => Some(GoogleVoice::KoreanKrWavenetBFemale),
            "ko-KR-Wavenet-C" => Some(GoogleVoice::KoreanKrWavenetCMale),
            "ko-KR-Wavenet-D" => Some(GoogleVoice::KoreanKrWavenetDMale),

            // Mandarin - China
            "cmn-CN-Standard-A" => Some(GoogleVoice::MandarinCnStandardAFemale),
            "cmn-CN-Standard-B" => Some(GoogleVoice::MandarinCnStandardBMale),
            "cmn-CN-Standard-C" => Some(GoogleVoice::MandarinCnStandardCMale),
            "cmn-CN-Standard-D" => Some(GoogleVoice::MandarinCnStandardDFemale),
            "cmn-CN-Wavenet-A" => Some(GoogleVoice::MandarinCnWavenetAFemale),
            "cmn-CN-Wavenet-B" => Some(GoogleVoice::MandarinCnWavenetBMale),
            "cmn-CN-Wavenet-C" => Some(GoogleVoice::MandarinCnWavenetCMale),
            "cmn-CN-Wavenet-D" => Some(GoogleVoice::MandarinCnWavenetDFemale),

            // Mandarin - Taiwan
            "cmn-TW-Standard-A-Alpha" => Some(GoogleVoice::MandarinTwStandardAAlpha),
            "cmn-TW-Standard-B-Alpha" => Some(GoogleVoice::MandarinTwStandardBAlpha),
            "cmn-TW-Standard-C-Alpha" => Some(GoogleVoice::MandarinTwStandardCAlpha),
            "cmn-TW-Wavenet-A-Alpha" => Some(GoogleVoice::MandarinTwWavenetAAlpha),
            "cmn-TW-Wavenet-B-Alpha" => Some(GoogleVoice::MandarinTwWavenetBAlpha),
            "cmn-TW-Wavenet-C-Alpha" => Some(GoogleVoice::MandarinTwWavenetCAlpha),

            // Norwegian
            "nb-NO-Standard-A" => Some(GoogleVoice::NorwegianNoStandardAFemale),
            "nb-NO-Standard-B" => Some(GoogleVoice::NorwegianNoStandardBMale),
            "nb-NO-Standard-C" => Some(GoogleVoice::NorwegianNoStandardCFemale),
            "nb-NO-Standard-D" => Some(GoogleVoice::NorwegianNoStandardDMale),
            "nb-NO-Wavenet-A" => Some(GoogleVoice::NorwegianNoWavenetAFemale),
            "nb-NO-Wavenet-B" => Some(GoogleVoice::NorwegianNoWavenetBMale),
            "nb-NO-Wavenet-C" => Some(GoogleVoice::NorwegianNoWavenetCFemale),
            "nb-NO-Wavenet-D" => Some(GoogleVoice::NorwegianNoWavenetDMale),

            // Polish (Note: enum has = instead of -)
            "pl-PL-Standard-A" => Some(GoogleVoice::PolishPlStandardAFemale),
            "pl-PL-Standard-B" => Some(GoogleVoice::PolishPlStandardBMale),
            "pl-PL-Standard-C" => Some(GoogleVoice::PolishPlStandardCMale),
            "pl-PL-Standard-D" => Some(GoogleVoice::PolishPlStandardDFemale),
            "pl-PL-Standard-E" => Some(GoogleVoice::PolishPlStandardEFemale),
            "pl-PL-Wavenet-A" => Some(GoogleVoice::PolishPlWavenetAFemale),
            "pl-PL-Wavenet-B" => Some(GoogleVoice::PolishPlWavenetBMale),
            "pl-PL-Wavenet-C" => Some(GoogleVoice::PolishPlWavenetCMale),
            "pl-PL-Wavenet-D" => Some(GoogleVoice::PolishPlWavenetDFemale),
            "pl-PL-Wavenet-E" => Some(GoogleVoice::PolishPlWavenetEFemale),

            // Portuguese - Brazil
            "pt-BR-Standard-A" => Some(GoogleVoice::PortugueseBrStandardAFemale),
            "pt-BR-Wavenet-A" => Some(GoogleVoice::PortugueseBrWavenetAFemale),

            // Portuguese - Portugal
            "pt-PT-Standard-A" => Some(GoogleVoice::PortuguesePtStandardAFemale),
            "pt-PT-Standard-B" => Some(GoogleVoice::PortuguesePtStandardBMale),
            "pt-PT-Standard-C" => Some(GoogleVoice::PortuguesePtStandardCMale),
            "pt-PT-Standard-D" => Some(GoogleVoice::PortuguesePtStandardDFemale),
            "pt-PT-Wavenet-A" => Some(GoogleVoice::PortuguesePtWavenetAFemale),
            "pt-PT-Wavenet-B" => Some(GoogleVoice::PortuguesePtWavenetBMale),
            "pt-PT-Wavenet-C" => Some(GoogleVoice::PortuguesePtWavenetCMale),
            "pt-PT-Wavenet-D" => Some(GoogleVoice::PortuguesePtWavenetDFemale),

            // Russian
            "ru-RU-Standard-A" => Some(GoogleVoice::RussianRuStandardAFemale),
            "ru-RU-Standard-B" => Some(GoogleVoice::RussianRuStandardBMale),
            "ru-RU-Standard-C" => Some(GoogleVoice::RussianRuStandardCFemale),
            "ru-RU-Standard-D" => Some(GoogleVoice::RussianRuStandardDMale),
            "ru-RU-Standard-E" => Some(GoogleVoice::RussianRuStandardEFemale),
            "ru-RU-Wavenet-A" => Some(GoogleVoice::RussianRuWavenetAFemale),
            "ru-RU-Wavenet-B" => Some(GoogleVoice::RussianRuWavenetBMale),
            "ru-RU-Wavenet-C" => Some(GoogleVoice::RussianRuWavenetCFemale),
            "ru-RU-Wavenet-D" => Some(GoogleVoice::RussianRuWavenetDMale),
            "ru-RU-Wavenet-E" => Some(GoogleVoice::RussianRuWavenetEFemale),

            // Slovak
            "sk-SK-Standard-A" => Some(GoogleVoice::SlovakSkStandardAFemale),
            "sk-SK-Wavenet-A" => Some(GoogleVoice::SlovakSkWavenetAFemale),

            // Spanish - Spain
            "es-ES-Standard-A" => Some(GoogleVoice::SpanishEsStandardAFemale),

            // Swedish
            "sv-SE-Standard-A" => Some(GoogleVoice::SwedishSeStandardAFemale),
            "sv-SE-Wavenet-A" => Some(GoogleVoice::SwedishSeWavenetAFemale),

            // Turkish
            "tr-TR-Standard-A" => Some(GoogleVoice::TurkishTrStandardAFemale),
            "tr-TR-Standard-B" => Some(GoogleVoice::TurkishTrStandardBMale),
            "tr-TR-Standard-C" => Some(GoogleVoice::TurkishTrStandardCFemale),
            "tr-TR-Standard-D" => Some(GoogleVoice::TurkishTrStandardDFemale),
            "tr-TR-Standard-E" => Some(GoogleVoice::TurkishTrStandardEMale),
            "tr-TR-Wavenet-A" => Some(GoogleVoice::TurkishTrWavenetAFemale),
            "tr-TR-Wavenet-B" => Some(GoogleVoice::TurkishTrWavenetBMale),
            "tr-TR-Wavenet-C" => Some(GoogleVoice::TurkishTrWavenetCFemale),
            "tr-TR-Wavenet-D" => Some(GoogleVoice::TurkishTrWavenetDFemale),
            "tr-TR-Wavenet-E" => Some(GoogleVoice::TurkishTrWavenetEMale),

            // Vietnamese
            "vi-VN-Standard-A" => Some(GoogleVoice::VietnameseVnStandardAFemale),
            "vi-VN-Standard-B" => Some(GoogleVoice::VietnameseVnStandardBMale),
            "vi-VN-Standard-C" => Some(GoogleVoice::VietnameseVnStandardCFemale),
            "vi-VN-Standard-D" => Some(GoogleVoice::VietnameseVnStandardDMale),
            "vi-VN-Wavenet-A" => Some(GoogleVoice::VietnameseVnWavenetAFemale),
            "vi-VN-Wavenet-B" => Some(GoogleVoice::VietnameseVnWavenetBMale),
            "vi-VN-Wavenet-C" => Some(GoogleVoice::VietnameseVnWavenetCFemale),
            "vi-VN-Wavenet-D" => Some(GoogleVoice::VietnameseVnWavenetDMale),

            _ => None,
        }
    }
}

impl GoogleTTSLanguage {
    /// Parses a voice string like "en-GB-Standard-A" and returns the language
    pub fn from_voice_string(voice: &str) -> Option<Self> {
        // Extract the language code (first 5 characters for most, but some are longer)
        if voice.len() < 5 {
            return None;
        }

        // Handle special cases first
        if voice.starts_with("yue-Hant-HK") {
            return Some(GoogleTTSLanguage::ChineseCantoneseTraditionalHongKong);
        }
        if voice.starts_with("pa-guru-IN") {
            return Some(GoogleTTSLanguage::PunjabiGurmukhiIndia);
        }
        if voice.starts_with("cmn-CN") {
            return Some(GoogleTTSLanguage::ChineseMandarinSimplifiedChina);
        }
        if voice.starts_with("cmn-TW") {
            return Some(GoogleTTSLanguage::ChineseMandarinSimplifiedTaiwan);
        }

        // For zh without suffix, it's simplified China
        if voice.starts_with("zh-") || voice == "zh" {
            if voice.starts_with("zh-HK") {
                return Some(GoogleTTSLanguage::ChineseMandarinSimplifiedHongKong);
            }
            if voice.starts_with("zh-TW") {
                return Some(GoogleTTSLanguage::ChineseMandarinSimplifiedTaiwan);
            }
            return Some(GoogleTTSLanguage::ChineseMandarinSimplifiedChina);
        }

        // Standard 5-character language codes
        let lang_code = &voice[0..5];

        match lang_code {
            // Afrikaans
            "af-ZA" => Some(GoogleTTSLanguage::AfrikaansSouthAfrica),

            // Albanian
            "sq-AL" => Some(GoogleTTSLanguage::AlbanianAlbania),

            // Amharic
            "am-ET" => Some(GoogleTTSLanguage::AmharicEthiopia),

            // Arabic
            "ar-DZ" => Some(GoogleTTSLanguage::ArabicAlgeria),
            "ar-BH" => Some(GoogleTTSLanguage::ArabicBahrain),
            "ar-EG" => Some(GoogleTTSLanguage::ArabicEgypt),
            "ar-IQ" => Some(GoogleTTSLanguage::ArabicIraq),
            "ar-IL" => Some(GoogleTTSLanguage::ArabicIsrael),
            "ar-JO" => Some(GoogleTTSLanguage::ArabicJordan),
            "ar-KW" => Some(GoogleTTSLanguage::ArabicKuwait),
            "ar-LB" => Some(GoogleTTSLanguage::ArabicLebanon),
            "ar-MA" => Some(GoogleTTSLanguage::ArabicMorocco),
            "ar-OM" => Some(GoogleTTSLanguage::ArabicOman),
            "ar-QA" => Some(GoogleTTSLanguage::ArabicQatar),
            "ar-SA" => Some(GoogleTTSLanguage::ArabicSaudiArabia),
            "ar-PS" => Some(GoogleTTSLanguage::ArabicStateofPalestine),
            "ar-TN" => Some(GoogleTTSLanguage::ArabicTunisia),
            "ar-AE" => Some(GoogleTTSLanguage::ArabicUnitedArabEmirates),
            "ar-XA" => Some(GoogleTTSLanguage::ArabicAlgeria), // ar-XA is a generic Arabic, defaulting to Algeria

            // Armenian
            "hy-AM" => Some(GoogleTTSLanguage::ArmenianArmenia),

            // Azerbaijani
            "az-AZ" => Some(GoogleTTSLanguage::AzerbaijaniAzerbaijan),

            // Basque
            "eu-ES" => Some(GoogleTTSLanguage::BasqueSpain),

            // Bengali
            "bn-BD" => Some(GoogleTTSLanguage::BengaliBangladesh),
            "bn-IN" => Some(GoogleTTSLanguage::BengaliIndia),

            // Bulgarian
            "bg-BG" => Some(GoogleTTSLanguage::BulgarianBulgaria),

            // Burmese
            "my-MM" => Some(GoogleTTSLanguage::BurmeseMyanmar),

            // Catalan
            "ca-ES" => Some(GoogleTTSLanguage::CatalanSpain),

            // Croatian
            "hr-HR" => Some(GoogleTTSLanguage::CroatianCroatia),

            // Czech
            "cs-CZ" => Some(GoogleTTSLanguage::CzechCzechRepublic),

            // Danish
            "da-DK" => Some(GoogleTTSLanguage::DanishDenmark),

            // Dutch
            "nl-BE" => Some(GoogleTTSLanguage::DutchBelgium),
            "nl-NL" => Some(GoogleTTSLanguage::DutchNetherlands),

            // English
            "en-AU" => Some(GoogleTTSLanguage::EnglishAustralia),
            "en-CA" => Some(GoogleTTSLanguage::EnglishCanada),
            "en-GH" => Some(GoogleTTSLanguage::EnglishGhana),
            "en-IN" => Some(GoogleTTSLanguage::EnglishIndia),
            "en-IE" => Some(GoogleTTSLanguage::EnglishIreland),
            "en-KE" => Some(GoogleTTSLanguage::EnglishKenya),
            "en-NZ" => Some(GoogleTTSLanguage::EnglishNewZealand),
            "en-NG" => Some(GoogleTTSLanguage::EnglishNigeria),
            "en-PH" => Some(GoogleTTSLanguage::EnglishPhilippines),
            "en-SG" => Some(GoogleTTSLanguage::EnglishSingapore),
            "en-ZA" => Some(GoogleTTSLanguage::EnglishSouthAfrica),
            "en-TZ" => Some(GoogleTTSLanguage::EnglishTanzania),
            "en-GB" => Some(GoogleTTSLanguage::EnglishUnitedKingdom),
            "en-US" => Some(GoogleTTSLanguage::EnglishUnitedStates),

            // Estonian
            "et-EE" => Some(GoogleTTSLanguage::EstonianEstonia),

            // Filipino
            "fil-P" => Some(GoogleTTSLanguage::FilipinoPhilippines), // fil-PH

            // Finnish
            "fi-FI" => Some(GoogleTTSLanguage::FinnishFinland),

            // French
            "fr-CA" => Some(GoogleTTSLanguage::FrenchCanada),
            "fr-FR" => Some(GoogleTTSLanguage::FrenchFrance),

            // Galician
            "gl-ES" => Some(GoogleTTSLanguage::GalicianSpain),

            // Georgian
            "ka-GE" => Some(GoogleTTSLanguage::GeorgianGeorgia),

            // German
            "de-DE" => Some(GoogleTTSLanguage::GermanGermany),

            // Greek
            "el-GR" => Some(GoogleTTSLanguage::GreekGreece),

            // Gujarati
            "gu-IN" => Some(GoogleTTSLanguage::GujaratiIndia),

            // Hebrew
            "he-IL" => Some(GoogleTTSLanguage::HebrewIsrael),

            // Hindi
            "hi-IN" => Some(GoogleTTSLanguage::HindiIndia),

            // Hungarian
            "hu-HU" => Some(GoogleTTSLanguage::HungarianHungary),

            // Icelandic
            "is-IS" => Some(GoogleTTSLanguage::IcelandicIceland),

            // Indonesian
            "id-ID" => Some(GoogleTTSLanguage::IndonesianIndonesia),

            // Italian
            "it-IT" => Some(GoogleTTSLanguage::ItalianItaly),

            // Japanese
            "ja-JP" => Some(GoogleTTSLanguage::JapaneseJapan),
            "js-JP" => Some(GoogleTTSLanguage::JapaneseJapan), // Handle typo in voice enum

            // Javanese
            "jv-ID" => Some(GoogleTTSLanguage::JavaneseIndonesia),

            // Kannada
            "kn-IN" => Some(GoogleTTSLanguage::KannadaIndia),

            // Khmer
            "km-KH" => Some(GoogleTTSLanguage::KhmerCambodia),

            // Korean
            "ko-KR" => Some(GoogleTTSLanguage::KoreanSouthKorea),

            // Lao
            "lo-LA" => Some(GoogleTTSLanguage::LaoLaos),

            // Latvian
            "lv-LV" => Some(GoogleTTSLanguage::LatvianLatvia),

            // Lithuanian
            "lt-LT" => Some(GoogleTTSLanguage::LithuanianLithuania),

            // Macedonian
            "mk-MK" => Some(GoogleTTSLanguage::MacedonianNorthMacedonia),

            // Malay
            "ms-MY" => Some(GoogleTTSLanguage::MalayMalaysia),

            // Malayalam
            "ml-IN" => Some(GoogleTTSLanguage::MalayalamIndia),

            // Marathi
            "mr-IN" => Some(GoogleTTSLanguage::MarathiIndia),

            // Mongolian
            "mn-MN" => Some(GoogleTTSLanguage::MongolianMongolia),

            // Nepali
            "ne-NP" => Some(GoogleTTSLanguage::NepaliNepal),

            // Norwegian
            "nb-NO" => Some(GoogleTTSLanguage::NorwegianBokmalNorway),

            // Persian
            "fa-IR" => Some(GoogleTTSLanguage::PersianIran),

            // Polish
            "pl-PL" => Some(GoogleTTSLanguage::PolishPoland),
            "pl=PL" => Some(GoogleTTSLanguage::PolishPoland), // Handle typo in voice enum

            // Portuguese
            "pt-BR" => Some(GoogleTTSLanguage::PortugueseBrazil),
            "pt-PT" => Some(GoogleTTSLanguage::PortuguesePortugal),

            // Romanian
            "ro-RO" => Some(GoogleTTSLanguage::RomanianRomania),

            // Russian
            "ru-RU" => Some(GoogleTTSLanguage::RussianRussia),

            // Serbian
            "sr-RS" => Some(GoogleTTSLanguage::SerbianSerbia),

            // Sinhala
            "si-LK" => Some(GoogleTTSLanguage::SinhalaSriLanka),

            // Slovak
            "sk-SK" => Some(GoogleTTSLanguage::SlovakSlovakia),

            // Slovenian
            "sl-SI" => Some(GoogleTTSLanguage::SlovenianSlovenia),

            // Spanish
            "es-AR" => Some(GoogleTTSLanguage::SpanishArgentina),
            "es-BO" => Some(GoogleTTSLanguage::SpanishBolivia),
            "es-CL" => Some(GoogleTTSLanguage::SpanishChile),
            "es-CO" => Some(GoogleTTSLanguage::SpanishColombia),
            "es-CR" => Some(GoogleTTSLanguage::SpanishCostaRica),
            "es-DO" => Some(GoogleTTSLanguage::SpanishDominicanRepublic),
            "es-EC" => Some(GoogleTTSLanguage::SpanishEcuador),
            "es-SV" => Some(GoogleTTSLanguage::SpanishElSalvador),
            "es-GT" => Some(GoogleTTSLanguage::SpanishGuatemala),
            "es-HN" => Some(GoogleTTSLanguage::SpanishHonduras),
            "es-MX" => Some(GoogleTTSLanguage::SpanishMexico),
            "es-NI" => Some(GoogleTTSLanguage::SpanishNicaragua),
            "es-PA" => Some(GoogleTTSLanguage::SpanishPanama),
            "es-PY" => Some(GoogleTTSLanguage::SpanishParaguay),
            "es-PE" => Some(GoogleTTSLanguage::SpanishPeru),
            "es-PR" => Some(GoogleTTSLanguage::SpanishPuertoRico),
            "es-ES" => Some(GoogleTTSLanguage::SpanishSpain),
            "es-US" => Some(GoogleTTSLanguage::SpanishUnitedStates),
            "es-UY" => Some(GoogleTTSLanguage::SpanishUruguay),
            "es-VE" => Some(GoogleTTSLanguage::SpanishVenezuela),

            // Sundanese
            "su-ID" => Some(GoogleTTSLanguage::SundaneseIndonesia),

            // Swahili
            "sw-KE" => Some(GoogleTTSLanguage::SwahiliKenya),
            "sw-TZ" => Some(GoogleTTSLanguage::SwahiliTanzania),

            // Swedish
            "sv-SE" => Some(GoogleTTSLanguage::SwedishSweden),

            // Tamil
            "ta-IN" => Some(GoogleTTSLanguage::TamilIndia),
            "ta-MY" => Some(GoogleTTSLanguage::TamilMalaysia),
            "ta-SG" => Some(GoogleTTSLanguage::TamilSingapore),
            "ta-LK" => Some(GoogleTTSLanguage::TamilSriLanka),

            // Telugu
            "te-IN" => Some(GoogleTTSLanguage::TeluguIndia),

            // Thai
            "th-TH" => Some(GoogleTTSLanguage::ThaiThailand),

            // Turkish
            "tr-TR" => Some(GoogleTTSLanguage::TurkishTurkey),

            // Ukrainian
            "uk-UA" => Some(GoogleTTSLanguage::UkrainianUkraine),

            // Urdu
            "ur-IN" => Some(GoogleTTSLanguage::UrduIndia),
            "ur-PK" => Some(GoogleTTSLanguage::UrduPakistan),

            // Uzbek
            "uz-UZ" => Some(GoogleTTSLanguage::UzbekUzbekistan),

            // Vietnamese
            "vi-VN" => Some(GoogleTTSLanguage::VietnameseVietnam),

            // Zulu
            "zu-ZA" => Some(GoogleTTSLanguage::ZuluSouthAfrica),

            _ => None,
        }
    }
}