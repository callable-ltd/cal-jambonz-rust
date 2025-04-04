use crate::verbs::vendors::vad::Vad;
use serde::{Deserialize, Serialize};
use crate::synthesizer::Synthesizer;

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleRecognizer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad: Option<Vad>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interim: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<GoogleRecognizerLanguage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_languages: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr_dtmf_termination_digit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr_timeout: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarization: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarization_min_speakers: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diarization_max_speakers: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enhanced_model: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hints_boost: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interaction_type: Option<GoogleInteractionType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<GoogleSpeechModel>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub naics_code: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub punctuation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_utterance: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate_recognition_per_channel: Option<bool>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GoogleSynthesizer {

    pub language: GoogleTTSLanguage,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice: Option<GoogleVoice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<GoogleGender>,

    
}

impl GoogleSynthesizer {
    pub fn voice(&mut self, voice:Option<GoogleVoice>) -> &mut  GoogleSynthesizer {
        self.voice = voice;
        self
    }

    pub fn gender(&mut self, gender: Option<GoogleGender>) -> &mut GoogleSynthesizer {
        self.gender = gender;
        self
    }
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GoogleSpeechModel {
    PhoneCall,
    Telephony,
    TelephonyShort,
    MedicalDictation,
    MedialConversation,
    LatestShort,
    LatestLong,
    CommandAndSearch,
    Default,
    Video,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GoogleGender {
    Male,
    Female,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum GoogleInteractionType {
    Discussion,
    Presentation,
    PhoneCall,
    VoiceMail,
    ProfessionallyProduced,
    VoiceSearch,
    VoiceCommand,
    Dictation,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GoogleVoice {
    #[serde(rename = "ar-XA-Standard-A")]
    ArabicStandardAFemale,
    #[serde(rename = "ar-XA-Standard-B")]
    ArabicStandardBMale,
    #[serde(rename = "ar-XA-Standard-C")]
    ArabicStandardCMale,
    #[serde(rename = "ar-XA-Standard-D")]
    ArabicStandardDFemale,
    #[serde(rename = "ar-XA-Wavenet-A")]
    ArabicWavenetAFemale,
    #[serde(rename = "ar-XA-Wavenet-B")]
    ArabicWavenetBMale,
    #[serde(rename = "ar-XA-Wavenet-C")]
    ArabicWavenetCMale,
    #[serde(rename = "cs-CZ-Standard-A")]
    CzechStandardAFemale,
    #[serde(rename = "cs-CZ-Wavenet-A")]
    CzechWavenetAFemale,
    #[serde(rename = "da-DK-Standard-A")]
    DanishStandardAFemale,
    #[serde(rename = "da-DK-Wavenet-A")]
    DanishWavenetAFemale,
    #[serde(rename = "nl-NL-Standard-A")]
    DutchStandardAFemale,
    #[serde(rename = "nl-NL-Standard-B")]
    DutchStandardBMale,
    #[serde(rename = "nl-NL-Standard-C")]
    DutchStandardCMale,
    #[serde(rename = "nl-NL-Standard-D")]
    DutchStandardDFemale,
    #[serde(rename = "nl-NL-Standard-E")]
    DutchStandardEFemale,
    #[serde(rename = "nl-NL-Wavenet-A")]
    DutchWavenetAFemale,
    #[serde(rename = "nl-NL-Wavenet-B")]
    DutchWavenetBMale,
    #[serde(rename = "nl-NL-Wavenet-C")]
    DutchWavenetCMale,
    #[serde(rename = "nl-NL-Wavenet-D")]
    DutchWavenetDFemale,
    #[serde(rename = "nl-NL-Wavenet-E")]
    DutchWavenetEFemale,
    #[serde(rename = "en-AU-Standard-A")]
    EnglishAuStandardAFemale,
    #[serde(rename = "en-AU-Standard-B")]
    EnglishAuStandardBMale,
    #[serde(rename = "en-AU-Standard-C")]
    EnglishAuStandardCFemale,
    #[serde(rename = "en-AU-Standard-D")]
    EnglishAuStandardDMale,
    #[serde(rename = "en-AU-Wavenet-A")]
    EnglishAuWavenetAFemale,
    #[serde(rename = "en-AU-Wavenet-B")]
    EnglishAuWavenetBMale,
    #[serde(rename = "en-AU-Wavenet-C")]
    EnglishAuWavenetCFemale,
    #[serde(rename = "en-AU-Wavenet-D")]
    EnglishAuWavenetDMale,
    #[serde(rename = "en-IN-Standard-A")]
    EnglishIndiaStandardAFemale,
    #[serde(rename = "en-IN-Standard-B")]
    EnglishIndiaStandardBMale,
    #[serde(rename = "en-IN-Standard-C")]
    EnglishIndiaStandardCMale,
    #[serde(rename = "en-IN-Standard-D")]
    EnglishIndiaStandardDFemale,
    #[serde(rename = "en-IN-Wavenet-A")]
    EnglishIndiaWavenetAFemale,
    #[serde(rename = "en-IN-Wavenet-B")]
    EnglishIndiaWavenetBMale,
    #[serde(rename = "en-IN-Wavenet-C")]
    EnglishIndiaWavenetCMale,
    #[serde(rename = "en-IN-Wavenet-D")]
    EnglishIndiaWavenetDFemale,
    #[serde(rename = "en-GB-Chirp-HD-D")]
    EnglishUkChirpHdDMale,
    #[serde(rename = "en-GB-Chirp-HD-F")]
    EnglishUkChirpHdFFemale,
    #[serde(rename = "en-GB-Chirp-HD-O")]
    EnglishUkChirpHdOFemale,
    #[serde(rename = "en-GB-News-G")]
    EnglishUkNewsGFemale,
    #[serde(rename = "en-GB-News-H")]
    EnglishUkNewsHFemale,
    #[serde(rename = "en-GB-News-I")]
    EnglishUkNewsIFemale,
    #[serde(rename = "en-GB-News-J")]
    EnglishUkNewsJMale,
    #[serde(rename = "en-GB-News-K")]
    EnglishUkNewsKMale,
    #[serde(rename = "en-GB-News-L")]
    EnglishUkNewsLMale,
    #[serde(rename = "en-GB-News-M")]
    EnglishUkNewsMMale,
    #[serde(rename = "en-GB-Standard-A")]
    EnglishUkStandardAFemale,
    #[serde(rename = "en-GB-Standard-B")]
    EnglishUkStandardBMale,
    #[serde(rename = "en-GB-Standard-C")]
    EnglishUkStandardCFemale,
    #[serde(rename = "en-GB-Standard-D")]
    EnglishUkStandardDMale,
    #[serde(rename = "en-GB-Standard-F")]
    EnglishUkStandardFFemale,
    #[serde(rename = "en-GB-Standard-N")]
    EnglishUkStandardNFemale,
    #[serde(rename = "en-GB-Standard-O")]
    EnglishUkStandardOMale,
    #[serde(rename = "en-GB-Studio-B")]
    EnglishUkStudioBMale,
    #[serde(rename = "en-GB-Studio-C")]
    EnglishUkStudioCFemale,
    #[serde(rename = "en-GB-Wavenet-A")]
    EnglishUkWavenetAFemale,
    #[serde(rename = "en-GB-Wavenet-B")]
    EnglishUkWavenetBMale,
    #[serde(rename = "en-GB-Wavenet-C")]
    EnglishUkWavenetCFemale,
    #[serde(rename = "en-GB-Wavenet-D")]
    EnglishUkWavenetDMale,
    #[serde(rename = "en-GB-Wavenet-F")]
    EnglishUkWavenetFFemale,
    #[serde(rename = "en-GB-Wavenet-N")]
    EnglishUkWavenetNFemale,
    #[serde(rename = "en-GB-Wavenet-N")]
    EnglishUkWavenetOMale,
    #[serde(rename = "en-GB-Neural2-N")]
    EnglishUkNeural2N,
    #[serde(rename = "en-GB-Neural2-O")]
    EnglishUkNeural2O,
    #[serde(rename = "en-US-Standard-B")]
    EnglishUsStandardBMale,
    #[serde(rename = "en-US-Standard-C")]
    EnglishUsStandardCFemale,
    #[serde(rename = "en-US-Standard-D")]
    EnglishUsStandardDMale,
    #[serde(rename = "en-US-Standard-E")]
    EnglishUsStandardEMale,
    #[serde(rename = "en-US-Wavenet-A")]
    EnglishUsWavenetAMale,
    #[serde(rename = "en-US-Wavenet-B")]
    EnglishUsWavenetBMale,
    #[serde(rename = "en-US-Wavenet-C")]
    EnglishUsWavenetCFemale,
    #[serde(rename = "en-US-Wavenet-D")]
    EnglishUsWavenetDMale,
    #[serde(rename = "en-US-Wavenet-E")]
    EnglishUsWavenetEFemale,
    #[serde(rename = "en-US-Wavenet-F")]
    EnglishUsWavenetFFemale,
    #[serde(rename = "fil-PH-Standard-A")]
    FilipinoPhStandardAFemale,
    #[serde(rename = "fil-PH-Wavenet-A")]
    FilipinoPhWavenetAFemale,
    #[serde(rename = "fi-FI-Standard-A")]
    FinnishFiStandardAFemale,
    #[serde(rename = "fi-FI-Wavenet-A")]
    FinnishFiWavenetAFemale,
    #[serde(rename = "fr-CA-Standard-A")]
    FrenchCaStandardAFemale,
    #[serde(rename = "fr-CA-Standard-B")]
    FrenchCaStandardBMale,
    #[serde(rename = "fr-CA-Standard-C")]
    FrenchCaStandardCFemale,
    #[serde(rename = "fr-CA-Standard-D")]
    FrenchCaStandardDMale,
    #[serde(rename = "fr-CA-Wavenet-A")]
    FrenchCaWavenetAFemale,
    #[serde(rename = "fr-CA-Wavenet-B")]
    FrenchCaWavenetBMale,
    #[serde(rename = "fr-CA-Wavenet-C")]
    FrenchCaWavenetCFemale,
    #[serde(rename = "fr-CA-Wavenet-D")]
    FrenchCaWavenetDMale,
    #[serde(rename = "fr-FR-Standard-A")]
    FrenchFrStandardAFemale,
    #[serde(rename = "fr-FR-Standard-B")]
    FrenchFrStandardBMale,
    #[serde(rename = "fr-FR-Standard-C")]
    FrenchFrStandardCFemale,
    #[serde(rename = "fr-FR-Standard-D")]
    FrenchFrStandardDMale,
    #[serde(rename = "fr-FR-Standard-E")]
    FrenchFrStandardEFemale,
    #[serde(rename = "fr-FR-Wavenet-A")]
    FrenchFrWavenetAFemale,
    #[serde(rename = "fr-FR-Wavenet-B")]
    FrenchFrWavenetBMale,
    #[serde(rename = "fr-FR-Wavenet-C")]
    FrenchFrWavenetCFemale,
    #[serde(rename = "fr-FR-Wavenet-D")]
    FrenchFrWavenetDMale,
    #[serde(rename = "fr-FR-Wavenet-E")]
    FrenchFrWavenetEFemale,
    #[serde(rename = "de-DE-Standard-A")]
    GermanDeStandardAFemale,
    #[serde(rename = "de-DE-Standard-B")]
    GermanDeStandardBMale,
    #[serde(rename = "de-DE-Standard-C")]
    GermanDeStandardEMale,
    #[serde(rename = "de-DE-Standard-D")]
    GermanDeStandardFFemale,
    #[serde(rename = "de-DE-Wavenet-A")]
    GermanDeWavenetAFemale,
    #[serde(rename = "de-DE-Wavenet-B")]
    GermanDeWavenetBMale,
    #[serde(rename = "de-DE-Wavenet-C")]
    GermanDeWavenetCFemale,
    #[serde(rename = "de-DE-Wavenet-D")]
    GermanDeWavenetDMale,
    #[serde(rename = "de-DE-Wavenet-E")]
    GermanDeWavenetEMale,
    #[serde(rename = "de-DE-Wavenet-F")]
    GermanDeWavenetFFemale,
    #[serde(rename = "el-GR-Standard-A")]
    GreekGrStandardAFemale,
    #[serde(rename = "el-GR-Wavenet-A")]
    GreekGrWavenetAFemale,
    #[serde(rename = "hi-IN-Standard-A")]
    HindiIndiaStandardAFemale,
    #[serde(rename = "hi-IN-Standard-B")]
    HindiIndiaStandardBMale,
    #[serde(rename = "hi-IN-Standard-C")]
    HindiIndiaStandardCMale,
    #[serde(rename = "hi-IN-Standard-D")]
    HindiIndiaStandardDFemale,
    #[serde(rename = "hi-IN-Wavenet-A")]
    HindiIndiaWavenetAFemale,
    #[serde(rename = "hi-IN-Wavenet-B")]
    HindiIndiaWavenetBMale,
    #[serde(rename = "hi-IN-Wavenet-C")]
    HindiIndiaWavenetCMale,
    #[serde(rename = "hi-IN-Wavenet-D")]
    HindiIndiaWavenetDFemale,
    #[serde(rename = "hu-HU-Standard-A")]
    HungarianHuStandardAFemale,
    #[serde(rename = "hu-HU-Wavenet-A")]
    HungarianHuWavenetAFemale,
    #[serde(rename = "id-ID-Standard-A")]
    IndonesianIdStandardAFemale,
    #[serde(rename = "id-ID-Standard-B")]
    IndonesianIdStandardBMale,
    #[serde(rename = "id-ID-Standard-C")]
    IndonesianIdStandardCMale,
    #[serde(rename = "id-ID-Standard-D")]
    IndonesianIdStandardDFemale,
    #[serde(rename = "id-ID-Wavenet-A")]
    IndonesianIdWavenetAFemale,
    #[serde(rename = "id-ID-Wavenet-B")]
    IndonesianIdWavenetBMale,
    #[serde(rename = "id-ID-Wavenet-C")]
    IndonesianIdWavenetCMale,
    #[serde(rename = "id-ID-Wavenet-D")]
    IndonesianIdWavenetDFemale,
    #[serde(rename = "it-IT-Standard-A")]
    ItalianItStandardAFemale,
    #[serde(rename = "it-IT-Standard-B")]
    ItalianItStandardBFemale,
    #[serde(rename = "it-IT-Standard-C")]
    ItalianItStandardCMale,
    #[serde(rename = "it-IT-Standard-D")]
    ItalianItStandardDMale,
    #[serde(rename = "it-IT-Wavenet-A")]
    ItalianItWavenetAFemale,
    #[serde(rename = "it-IT-Wavenet-B")]
    ItalianItWavenetBFemale,
    #[serde(rename = "it-IT-Wavenet-C")]
    ItalianItWavenetCMale,
    #[serde(rename = "it-IT-Wavenet-D")]
    ItalianItWavenetDMale,
    #[serde(rename = "js-JP-Standard-A")]
    JapaneseJpStandardAFemale,
    #[serde(rename = "js-JP-Standard-B")]
    JapaneseJpStandardBFemale,
    #[serde(rename = "js-JP-Standard-C")]
    JapaneseJpStandardCMale,
    #[serde(rename = "js-JP-Standard-D")]
    JapaneseJpStandardDMale,
    #[serde(rename = "js-JP-Wavenet-A")]
    JapaneseJpWavenetAFemale,
    #[serde(rename = "js-JP-Wavenet-B")]
    JapaneseJpWavenetBFemale,
    #[serde(rename = "js-JP-Wavenet-C")]
    JapaneseJpWavenetCMale,
    #[serde(rename = "js-JP-Wavenet-D")]
    JapaneseJpWavenetDMale,
    #[serde(rename = "ko-KR-Standard-A")]
    KoreanKrStandardAFemale,
    #[serde(rename = "ko-KR-Standard-B")]
    KoreanKrStandardBFemale,
    #[serde(rename = "ko-KR-Standard-C")]
    KoreanKrStandardCMale,
    #[serde(rename = "ko-KR-Standard-D")]
    KoreanKrStandardDMale,
    #[serde(rename = "ko-KR-Wavenet-A")]
    KoreanKrWavenetAFemale,
    #[serde(rename = "ko-KR-Wavenet-B")]
    KoreanKrWavenetBFemale,
    #[serde(rename = "ko-KR-Wavenet-C")]
    KoreanKrWavenetCMale,
    #[serde(rename = "ko-KR-Wavenet-D")]
    KoreanKrWavenetDMale,
    #[serde(rename = "cmn-CN-Standard-A")]
    MandarinCnStandardAFemale,
    #[serde(rename = "cmn-CN-Standard-B")]
    MandarinCnStandardBMale,
    #[serde(rename = "cmn-CN-Standard-C")]
    MandarinCnStandardCMale,
    #[serde(rename = "cmn-CN-Standard-D")]
    MandarinCnStandardDFemale,
    #[serde(rename = "cmn-CN-Wavenet-A")]
    MandarinCnWavenetAFemale,
    #[serde(rename = "cmn-CN-Wavenet-B")]
    MandarinCnWavenetBMale,
    #[serde(rename = "cmn-CN-Wavenet-C")]
    MandarinCnWavenetCMale,
    #[serde(rename = "cmn-CN-Wavenet-D")]
    MandarinCnWavenetDFemale,
    #[serde(rename = "cmn-TW-Standard-A-Alpha")]
    MandarinTwStandardAAlpha,
    #[serde(rename = "cmn-TW-Standard-B-Alpha")]
    MandarinTwStandardBAlpha,
    #[serde(rename = "cmn-TW-Standard-C-Alpha")]
    MandarinTwStandardCAlpha,
    #[serde(rename = "cmn-TW-Wavenet-A-Alpha")]
    MandarinTwWavenetAAlpha,
    #[serde(rename = "cmn-TW-Wavenet-B-Alpha")]
    MandarinTwWavenetBAlpha,
    #[serde(rename = "cmn-TW-Wavenet-C-Alpha")]
    MandarinTwWavenetCAlpha,
    #[serde(rename = "nb-NO-Standard-A")]
    NorwegianNoStandardAFemale,
    #[serde(rename = "nb-NO-Standard-B")]
    NorwegianNoStandardBMale,
    #[serde(rename = "nb-NO-Standard-C")]
    NorwegianNoStandardCFemale,
    #[serde(rename = "nb-NO-Standard-D")]
    NorwegianNoStandardDMale,
    #[serde(rename = "nb-NO-Wavenet-A")]
    NorwegianNoWavenetAFemale,
    #[serde(rename = "nb-NO-Wavenet-B")]
    NorwegianNoWavenetBMale,
    #[serde(rename = "nb-NO-Wavenet-C")]
    NorwegianNoWavenetCFemale,
    #[serde(rename = "nb-NO-Wavenet-D")]
    NorwegianNoWavenetDMale,
    #[serde(rename = "pl=PL-Standard-A")]
    PolishPlStandardAFemale,
    #[serde(rename = "pl=PL-Standard-B")]
    PolishPlStandardBMale,
    #[serde(rename = "pl=PL-Standard-C")]
    PolishPlStandardCMale,
    #[serde(rename = "pl=PL-Standard-D")]
    PolishPlStandardDFemale,
    #[serde(rename = "pl=PL-Standard-E")]
    PolishPlStandardEFemale,
    #[serde(rename = "pl=PL-Wavenet-A")]
    PolishPlWavenetAFemale,
    #[serde(rename = "pl=PL-Wavenet-B")]
    PolishPlWavenetBMale,
    #[serde(rename = "pl=PL-Wavenet-C")]
    PolishPlWavenetCMale,
    #[serde(rename = "pl=PL-Wavenet-D")]
    PolishPlWavenetDFemale,
    #[serde(rename = "pl=PL-Wavenet-E")]
    PolishPlWavenetEFemale,
    #[serde(rename = "pt-BR-Standard-A")]
    PortugueseBrStandardAFemale,
    #[serde(rename = "pt-BR-Wavenet-A")]
    PortugueseBrWavenetAFemale,
    #[serde(rename = "pt-PT-Standard-A")]
    PortuguesePtStandardAFemale,
    #[serde(rename = "pt-PT-Standard-B")]
    PortuguesePtStandardBMale,
    #[serde(rename = "pt-PT-Standard-C")]
    PortuguesePtStandardCMale,
    #[serde(rename = "pt-PT-Standard-D")]
    PortuguesePtStandardDFemale,
    #[serde(rename = "pt-PT-Wavenet-A")]
    PortuguesePtWavenetAFemale,
    #[serde(rename = "pt-PT-Wavenet-B")]
    PortuguesePtWavenetBMale,
    #[serde(rename = "pt-PT-Wavenet-C")]
    PortuguesePtWavenetCMale,
    #[serde(rename = "pt-PT-Wavenet-D")]
    PortuguesePtWavenetDFemale,
    #[serde(rename = "ru-RU-Standard-A")]
    RussianRuStandardAFemale,
    #[serde(rename = "ru-RU-Standard-B")]
    RussianRuStandardBMale,
    #[serde(rename = "ru-RU-Standard-C")]
    RussianRuStandardCFemale,
    #[serde(rename = "ru-RU-Standard-D")]
    RussianRuStandardDMale,
    #[serde(rename = "ru-RU-Standard-E")]
    RussianRuStandardEFemale,
    #[serde(rename = "ru-RU-Wavenet-A")]
    RussianRuWavenetAFemale,
    #[serde(rename = "ru-RU-Wavenet-B")]
    RussianRuWavenetBMale,
    #[serde(rename = "ru-RU-Wavenet-C")]
    RussianRuWavenetCFemale,
    #[serde(rename = "ru-RU-Wavenet-D")]
    RussianRuWavenetDMale,
    #[serde(rename = "ru-RU-Wavenet-E")]
    RussianRuWavenetEFemale,
    #[serde(rename = "sk-SK-Standard-A")]
    SlovakSkStandardAFemale,
    #[serde(rename = "sk-SK-Wavenet-A")]
    SlovakSkWavenetAFemale,
    #[serde(rename = "es-ES-Standard-A")]
    SpanishEsStandardAFemale,
    #[serde(rename = "sv-SE-Standard-A")]
    SwedishSeStandardAFemale,
    #[serde(rename = "sv-SE-Wavenet-A")]
    SwedishSeWavenetAFemale,
    #[serde(rename = "tr-TR-Standard-A")]
    TurkishTrStandardAFemale,
    #[serde(rename = "tr-TR-Standard-B")]
    TurkishTrStandardBMale,
    #[serde(rename = "tr-TR-Standard-C")]
    TurkishTrStandardCFemale,
    #[serde(rename = "tr-TR-Standard-D")]
    TurkishTrStandardDFemale,
    #[serde(rename = "tr-TR-Standard-E")]
    TurkishTrStandardEMale,
    #[serde(rename = "tr-TR-Wavenet-A")]
    TurkishTrWavenetAFemale,
    #[serde(rename = "tr-TR-Wavenet-B")]
    TurkishTrWavenetBMale,
    #[serde(rename = "tr-TR-Wavenet-C")]
    TurkishTrWavenetCFemale,
    #[serde(rename = "tr-TR-Wavenet-D")]
    TurkishTrWavenetDFemale,
    #[serde(rename = "tr-TR-Wavenet-E")]
    TurkishTrWavenetEMale,
    #[serde(rename = "uk-UA-Standard-A")]
    UkranianUaStandardAFemale,
    #[serde(rename = "uk-UA-Wavenet-A")]
    UkranianUaWavenetAFemale,
    #[serde(rename = "vi-VN-Standard-A")]
    VietnameseVnStandardAFemale,
    #[serde(rename = "vi-VN-Standard-B")]
    VietnameseVnStandardBMale,
    #[serde(rename = "vi-VN-Standard-C")]
    VietnameseVnStandardCFemale,
    #[serde(rename = "vi-VN-Standard-D")]
    VietnameseVnStandardDMale,
    #[serde(rename = "vi-VN-Wavenet-A")]
    VietnameseVnWavenetAFemale,
    #[serde(rename = "vi-VN-Wavenet-B")]
    VietnameseVnWavenetBMale,
    #[serde(rename = "vi-VN-Wavenet-C")]
    VietnameseVnWavenetCFemale,
    #[serde(rename = "vi-VN-Wavenet-D")]
    VietnameseVnWavenetDMale,
}



// TODO This needs to reflect available TTS languages,
//  Currently copied from ASR Languages
//  Reference assets/google_tts_languages to build enum.
#[derive(Serialize, Deserialize, Clone)]
pub enum GoogleTTSLanguage {
    #[serde(rename = "af-ZA")]
    AfrikaansSouthAfrica,
    #[serde(rename = "sq-AL")]
    AlbanianAlbania,
    #[serde(rename = "am-ET")]
    AmharicEthiopia,
    #[serde(rename = "ar-DZ")]
    ArabicAlgeria,
    #[serde(rename = "ar-BH")]
    ArabicBahrain,
    #[serde(rename = "ar-EG")]
    ArabicEgypt,
    #[serde(rename = "ar-IQ")]
    ArabicIraq,
    #[serde(rename = "ar-IL")]
    ArabicIsrael,
    #[serde(rename = "ar-JO")]
    ArabicJordan,
    #[serde(rename = "ar-KW")]
    ArabicKuwait,
    #[serde(rename = "ar-LB")]
    ArabicLebanon,
    #[serde(rename = "ar-MA")]
    ArabicMorocco,
    #[serde(rename = "ar-OM")]
    ArabicOman,
    #[serde(rename = "ar-QA")]
    ArabicQatar,
    #[serde(rename = "ar-SA")]
    ArabicSaudiArabia,
    #[serde(rename = "ar-PS")]
    ArabicStateofPalestine,
    #[serde(rename = "ar-TN")]
    ArabicTunisia,
    #[serde(rename = "ar-AE")]
    ArabicUnitedArabEmirates,
    #[serde(rename = "hy-AM")]
    ArmenianArmenia,
    #[serde(rename = "az-AZ")]
    AzerbaijaniAzerbaijan,
    #[serde(rename = "eu-ES")]
    BasqueSpain,
    #[serde(rename = "bn-BD")]
    BengaliBangladesh,
    #[serde(rename = "bn-IN")]
    BengaliIndia,
    #[serde(rename = "bg-BG")]
    BulgarianBulgaria,
    #[serde(rename = "my-MM")]
    BurmeseMyanmar,
    #[serde(rename = "ca-ES")]
    CatalanSpain,
    #[serde(rename = "yue-Hant-HK")]
    ChineseCantoneseTraditionalHongKong,
    #[serde(rename = "zh")]
    ChineseMandarinSimplifiedChina,
    #[serde(rename = "zh-HK")]
    ChineseMandarinSimplifiedHongKong,
    #[serde(rename = "zh-TW")]
    ChineseMandarinSimplifiedTaiwan,
    #[serde(rename = "hr-HR")]
    CroatianCroatia,
    #[serde(rename = "cs-CZ")]
    CzechCzechRepublic,
    #[serde(rename = "da-DK")]
    DanishDenmark,
    #[serde(rename = "nl-BE")]
    DutchBelgium,
    #[serde(rename = "nl-NL")]
    DutchNetherlands,
    #[serde(rename = "en-AU")]
    EnglishAustralia,
    #[serde(rename = "en-CA")]
    EnglishCanada,
    #[serde(rename = "en-GH")]
    EnglishGhana,
    #[serde(rename = "en-IN")]
    EnglishIndia,
    #[serde(rename = "en-IE")]
    EnglishIreland,
    #[serde(rename = "en-KE")]
    EnglishKenya,
    #[serde(rename = "en-NZ")]
    EnglishNewZealand,
    #[serde(rename = "en-NG")]
    EnglishNigeria,
    #[serde(rename = "en-PH")]
    EnglishPhilippines,
    #[serde(rename = "en-SG")]
    EnglishSingapore,
    #[serde(rename = "en-ZA")]
    EnglishSouthAfrica,
    #[serde(rename = "en-TZ")]
    EnglishTanzania,
    #[serde(rename = "en-GB")]
    EnglishUnitedKingdom,
    #[serde(rename = "en-US")]
    EnglishUnitedStates,
    #[serde(rename = "et-EE")]
    EstonianEstonia,
    #[serde(rename = "fil-PH")]
    FilipinoPhilippines,
    #[serde(rename = "fi-FI")]
    FinnishFinland,
    #[serde(rename = "fr-CA")]
    FrenchCanada,
    #[serde(rename = "fr-FR")]
    FrenchFrance,
    #[serde(rename = "gl-ES")]
    GalicianSpain,
    #[serde(rename = "ka-GE")]
    GeorgianGeorgia,
    #[serde(rename = "de-DE")]
    GermanGermany,
    #[serde(rename = "el-GR")]
    GreekGreece,
    #[serde(rename = "gu-IN")]
    GujaratiIndia,
    #[serde(rename = "he-IL")]
    HebrewIsrael,
    #[serde(rename = "hi-IN")]
    HindiIndia,
    #[serde(rename = "hu-HU")]
    HungarianHungary,
    #[serde(rename = "is-IS")]
    IcelandicIceland,
    #[serde(rename = "id-ID")]
    IndonesianIndonesia,
    #[serde(rename = "it-IT")]
    ItalianItaly,
    #[serde(rename = "ja-JP")]
    JapaneseJapan,
    #[serde(rename = "jv-ID")]
    JavaneseIndonesia,
    #[serde(rename = "kn-IN")]
    KannadaIndia,
    #[serde(rename = "km-KH")]
    KhmerCambodia,
    #[serde(rename = "ko-KR")]
    KoreanSouthKorea,
    #[serde(rename = "lo-LA")]
    LaoLaos,
    #[serde(rename = "lv-LV")]
    LatvianLatvia,
    #[serde(rename = "lt-LT")]
    LithuanianLithuania,
    #[serde(rename = "mk-MK")]
    MacedonianNorthMacedonia,
    #[serde(rename = "ms-MY")]
    MalayMalaysia,
    #[serde(rename = "ml-IN")]
    MalayalamIndia,
    #[serde(rename = "mr-IN")]
    MarathiIndia,
    #[serde(rename = "mn-MN")]
    MongolianMongolia,
    #[serde(rename = "ne-NP")]
    NepaliNepal,
    #[serde(rename = "nb-NO")]
    NorwegianBokmalNorway,
    #[serde(rename = "fa-IR")]
    PersianIran,
    #[serde(rename = "pl-PL")]
    PolishPoland,
    #[serde(rename = "pt-BR")]
    PortugueseBrazil,
    #[serde(rename = "pt-PT")]
    PortuguesePortugal,
    #[serde(rename = "pa-guru-IN")]
    PunjabiGurmukhiIndia,
    #[serde(rename = "ro-RO")]
    RomanianRomania,
    #[serde(rename = "ru-RU")]
    RussianRussia,
    #[serde(rename = "sr-RS")]
    SerbianSerbia,
    #[serde(rename = "si-LK")]
    SinhalaSriLanka,
    #[serde(rename = "sk-SK")]
    SlovakSlovakia,
    #[serde(rename = "sl-SI")]
    SlovenianSlovenia,
    #[serde(rename = "es-AR")]
    SpanishArgentina,
    #[serde(rename = "es-BO")]
    SpanishBolivia,
    #[serde(rename = "es-CL")]
    SpanishChile,
    #[serde(rename = "es-CO")]
    SpanishColombia,
    #[serde(rename = "es-CR")]
    SpanishCostaRica,
    #[serde(rename = "es-DO")]
    SpanishDominicanRepublic,
    #[serde(rename = "es-EC")]
    SpanishEcuador,
    #[serde(rename = "es-SV")]
    SpanishElSalvador,
    #[serde(rename = "es-GT")]
    SpanishGuatemala,
    #[serde(rename = "es-HN")]
    SpanishHonduras,
    #[serde(rename = "es-MX")]
    SpanishMexico,
    #[serde(rename = "es-NI")]
    SpanishNicaragua,
    #[serde(rename = "es-PA")]
    SpanishPanama,
    #[serde(rename = "es-PY")]
    SpanishParaguay,
    #[serde(rename = "es-PE")]
    SpanishPeru,
    #[serde(rename = "es-PR")]
    SpanishPuertoRico,
    #[serde(rename = "es-ES")]
    SpanishSpain,
    #[serde(rename = "es-US")]
    SpanishUnitedStates,
    #[serde(rename = "es-UY")]
    SpanishUruguay,
    #[serde(rename = "es-VE")]
    SpanishVenezuela,
    #[serde(rename = "su-ID")]
    SundaneseIndonesia,
    #[serde(rename = "sw-KE")]
    SwahiliKenya,
    #[serde(rename = "sw-TZ")]
    SwahiliTanzania,
    #[serde(rename = "sv-SE")]
    SwedishSweden,
    #[serde(rename = "ta-IN")]
    TamilIndia,
    #[serde(rename = "ta-MY")]
    TamilMalaysia,
    #[serde(rename = "ta-SG")]
    TamilSingapore,
    #[serde(rename = "ta-LK")]
    TamilSriLanka,
    #[serde(rename = "te-IN")]
    TeluguIndia,
    #[serde(rename = "th-TH")]
    ThaiThailand,
    #[serde(rename = "tr-TR")]
    TurkishTurkey,
    #[serde(rename = "uk-UA")]
    UkrainianUkraine,
    #[serde(rename = "ur-IN")]
    UrduIndia,
    #[serde(rename = "ur-PK")]
    UrduPakistan,
    #[serde(rename = "uz-UZ")]
    UzbekUzbekistan,
    #[serde(rename = "vi-VN")]
    VietnameseVietnam,
    #[serde(rename = "zu-ZA")]
    ZuluSouthAfrica,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum GoogleRecognizerLanguage {
    #[serde(rename = "af-ZA")]
    AfrikaansSouthAfrica,
    #[serde(rename = "sq-AL")]
    AlbanianAlbania,
    #[serde(rename = "am-ET")]
    AmharicEthiopia,
    #[serde(rename = "ar-DZ")]
    ArabicAlgeria,
    #[serde(rename = "ar-BH")]
    ArabicBahrain,
    #[serde(rename = "ar-EG")]
    ArabicEgypt,
    #[serde(rename = "ar-IQ")]
    ArabicIraq,
    #[serde(rename = "ar-IL")]
    ArabicIsrael,
    #[serde(rename = "ar-JO")]
    ArabicJordan,
    #[serde(rename = "ar-KW")]
    ArabicKuwait,
    #[serde(rename = "ar-LB")]
    ArabicLebanon,
    #[serde(rename = "ar-MA")]
    ArabicMorocco,
    #[serde(rename = "ar-OM")]
    ArabicOman,
    #[serde(rename = "ar-QA")]
    ArabicQatar,
    #[serde(rename = "ar-SA")]
    ArabicSaudiArabia,
    #[serde(rename = "ar-PS")]
    ArabicStateofPalestine,
    #[serde(rename = "ar-TN")]
    ArabicTunisia,
    #[serde(rename = "ar-AE")]
    ArabicUnitedArabEmirates,
    #[serde(rename = "hy-AM")]
    ArmenianArmenia,
    #[serde(rename = "az-AZ")]
    AzerbaijaniAzerbaijan,
    #[serde(rename = "eu-ES")]
    BasqueSpain,
    #[serde(rename = "bn-BD")]
    BengaliBangladesh,
    #[serde(rename = "bn-IN")]
    BengaliIndia,
    #[serde(rename = "bg-BG")]
    BulgarianBulgaria,
    #[serde(rename = "my-MM")]
    BurmeseMyanmar,
    #[serde(rename = "ca-ES")]
    CatalanSpain,
    #[serde(rename = "yue-Hant-HK")]
    ChineseCantoneseTraditionalHongKong,
    #[serde(rename = "zh")]
    ChineseMandarinSimplifiedChina,
    #[serde(rename = "zh-HK")]
    ChineseMandarinSimplifiedHongKong,
    #[serde(rename = "zh-TW")]
    ChineseMandarinSimplifiedTaiwan,
    #[serde(rename = "hr-HR")]
    CroatianCroatia,
    #[serde(rename = "cs-CZ")]
    CzechCzechRepublic,
    #[serde(rename = "da-DK")]
    DanishDenmark,
    #[serde(rename = "nl-BE")]
    DutchBelgium,
    #[serde(rename = "nl-NL")]
    DutchNetherlands,
    #[serde(rename = "en-AU")]
    EnglishAustralia,
    #[serde(rename = "en-CA")]
    EnglishCanada,
    #[serde(rename = "en-GH")]
    EnglishGhana,
    #[serde(rename = "en-IN")]
    EnglishIndia,
    #[serde(rename = "en-IE")]
    EnglishIreland,
    #[serde(rename = "en-KE")]
    EnglishKenya,
    #[serde(rename = "en-NZ")]
    EnglishNewZealand,
    #[serde(rename = "en-NG")]
    EnglishNigeria,
    #[serde(rename = "en-PH")]
    EnglishPhilippines,
    #[serde(rename = "en-SG")]
    EnglishSingapore,
    #[serde(rename = "en-ZA")]
    EnglishSouthAfrica,
    #[serde(rename = "en-TZ")]
    EnglishTanzania,
    #[serde(rename = "en-GB")]
    EnglishUnitedKingdom,
    #[serde(rename = "en-US")]
    EnglishUnitedStates,
    #[serde(rename = "et-EE")]
    EstonianEstonia,
    #[serde(rename = "fil-PH")]
    FilipinoPhilippines,
    #[serde(rename = "fi-FI")]
    FinnishFinland,
    #[serde(rename = "fr-CA")]
    FrenchCanada,
    #[serde(rename = "fr-FR")]
    FrenchFrance,
    #[serde(rename = "gl-ES")]
    GalicianSpain,
    #[serde(rename = "ka-GE")]
    GeorgianGeorgia,
    #[serde(rename = "de-DE")]
    GermanGermany,
    #[serde(rename = "el-GR")]
    GreekGreece,
    #[serde(rename = "gu-IN")]
    GujaratiIndia,
    #[serde(rename = "he-IL")]
    HebrewIsrael,
    #[serde(rename = "hi-IN")]
    HindiIndia,
    #[serde(rename = "hu-HU")]
    HungarianHungary,
    #[serde(rename = "is-IS")]
    IcelandicIceland,
    #[serde(rename = "id-ID")]
    IndonesianIndonesia,
    #[serde(rename = "it-IT")]
    ItalianItaly,
    #[serde(rename = "ja-JP")]
    JapaneseJapan,
    #[serde(rename = "jv-ID")]
    JavaneseIndonesia,
    #[serde(rename = "kn-IN")]
    KannadaIndia,
    #[serde(rename = "km-KH")]
    KhmerCambodia,
    #[serde(rename = "ko-KR")]
    KoreanSouthKorea,
    #[serde(rename = "lo-LA")]
    LaoLaos,
    #[serde(rename = "lv-LV")]
    LatvianLatvia,
    #[serde(rename = "lt-LT")]
    LithuanianLithuania,
    #[serde(rename = "mk-MK")]
    MacedonianNorthMacedonia,
    #[serde(rename = "ms-MY")]
    MalayMalaysia,
    #[serde(rename = "ml-IN")]
    MalayalamIndia,
    #[serde(rename = "mr-IN")]
    MarathiIndia,
    #[serde(rename = "mn-MN")]
    MongolianMongolia,
    #[serde(rename = "ne-NP")]
    NepaliNepal,
    #[serde(rename = "nb-NO")]
    NorwegianBokmalNorway,
    #[serde(rename = "fa-IR")]
    PersianIran,
    #[serde(rename = "pl-PL")]
    PolishPoland,
    #[serde(rename = "pt-BR")]
    PortugueseBrazil,
    #[serde(rename = "pt-PT")]
    PortuguesePortugal,
    #[serde(rename = "pa-guru-IN")]
    PunjabiGurmukhiIndia,
    #[serde(rename = "ro-RO")]
    RomanianRomania,
    #[serde(rename = "ru-RU")]
    RussianRussia,
    #[serde(rename = "sr-RS")]
    SerbianSerbia,
    #[serde(rename = "si-LK")]
    SinhalaSriLanka,
    #[serde(rename = "sk-SK")]
    SlovakSlovakia,
    #[serde(rename = "sl-SI")]
    SlovenianSlovenia,
    #[serde(rename = "es-AR")]
    SpanishArgentina,
    #[serde(rename = "es-BO")]
    SpanishBolivia,
    #[serde(rename = "es-CL")]
    SpanishChile,
    #[serde(rename = "es-CO")]
    SpanishColombia,
    #[serde(rename = "es-CR")]
    SpanishCostaRica,
    #[serde(rename = "es-DO")]
    SpanishDominicanRepublic,
    #[serde(rename = "es-EC")]
    SpanishEcuador,
    #[serde(rename = "es-SV")]
    SpanishElSalvador,
    #[serde(rename = "es-GT")]
    SpanishGuatemala,
    #[serde(rename = "es-HN")]
    SpanishHonduras,
    #[serde(rename = "es-MX")]
    SpanishMexico,
    #[serde(rename = "es-NI")]
    SpanishNicaragua,
    #[serde(rename = "es-PA")]
    SpanishPanama,
    #[serde(rename = "es-PY")]
    SpanishParaguay,
    #[serde(rename = "es-PE")]
    SpanishPeru,
    #[serde(rename = "es-PR")]
    SpanishPuertoRico,
    #[serde(rename = "es-ES")]
    SpanishSpain,
    #[serde(rename = "es-US")]
    SpanishUnitedStates,
    #[serde(rename = "es-UY")]
    SpanishUruguay,
    #[serde(rename = "es-VE")]
    SpanishVenezuela,
    #[serde(rename = "su-ID")]
    SundaneseIndonesia,
    #[serde(rename = "sw-KE")]
    SwahiliKenya,
    #[serde(rename = "sw-TZ")]
    SwahiliTanzania,
    #[serde(rename = "sv-SE")]
    SwedishSweden,
    #[serde(rename = "ta-IN")]
    TamilIndia,
    #[serde(rename = "ta-MY")]
    TamilMalaysia,
    #[serde(rename = "ta-SG")]
    TamilSingapore,
    #[serde(rename = "ta-LK")]
    TamilSriLanka,
    #[serde(rename = "te-IN")]
    TeluguIndia,
    #[serde(rename = "th-TH")]
    ThaiThailand,
    #[serde(rename = "tr-TR")]
    TurkishTurkey,
    #[serde(rename = "uk-UA")]
    UkrainianUkraine,
    #[serde(rename = "ur-IN")]
    UrduIndia,
    #[serde(rename = "ur-PK")]
    UrduPakistan,
    #[serde(rename = "uz-UZ")]
    UzbekUzbekistan,
    #[serde(rename = "vi-VN")]
    VietnameseVietnam,
    #[serde(rename = "zu-ZA")]
    ZuluSouthAfrica,
}
