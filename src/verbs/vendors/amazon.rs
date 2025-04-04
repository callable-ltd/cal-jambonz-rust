use crate::verbs::vendors::vad::Vad;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AWSRecognizer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vad: Option<Vad>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interim: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<AwsAsrLanguage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alt_languages: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr_dtmf_termination_digit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub asr_timeout: Option<u8>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub separate_recognition_per_channel: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub identify_channels: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter_method: Option<AWSFilterMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub vocabulary_filter_name: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AWSFilterMethod {
    Remove,
    Mask,
    Tag,
}

//todo
#[derive(Serialize, Deserialize, Clone)]
pub struct AwsSynthesizer {
    
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AwsTTSLanguage {
    #[serde(rename = "arb")]
    Arabic,
    #[serde(rename = "ar-AE")]
    ArabicGulf,
    #[serde(rename = "ca-ES")]
    Catalan,
    #[serde(rename = "yue-CN")]
    ChineseCantonese,
    #[serde(rename = "cmn-CN")]
    ChineseMandarin,
    #[serde(rename = "cs-CZ")]
    Czech,
    #[serde(rename = "da-DK")]
    Danish,
    #[serde(rename = "nl-BE")]
    DutchBelgian,
    #[serde(rename = "nl-NL")]
    Dutch,
    #[serde(rename = "en-AU")]
    EnglishAustralian,
    #[serde(rename = "en-GB")]
    EnglishBritish,
    #[serde(rename = "en-IN")]
    EnglishIndian,
    #[serde(rename = "en-NZ")]
    EnglishNewZealand,
    #[serde(rename = "en-SG")]
    EnglishSingaporean,
    #[serde(rename = "en-ZA")]
    EnglishSouthAfrican,
    #[serde(rename = "en-US")]
    EnglishUS,
    #[serde(rename = "en-GB-WLS")]
    EnglishWelsh,
    #[serde(rename = "fi-FI")]
    Finnish,
    #[serde(rename = "fr-FR")]
    French,
    #[serde(rename = "fr-BE")]
    FrenchBelgian,
    #[serde(rename = "fr-CA")]
    FrenchCanadian,
    #[serde(rename = "hi-IN")]
    Hindi,
    #[serde(rename = "de-DE")]
    German,
    #[serde(rename = "de-AT")]
    GermanAustrian,
    #[serde(rename = "de-CH")]
    GermanSwissstandard,
    #[serde(rename = "is-IS")]
    Icelandic,
    #[serde(rename = "it-IT")]
    Italian,
    #[serde(rename = "ja-JP")]
    Japanese,
    #[serde(rename = "ko-KR")]
    Korean,
    #[serde(rename = "nb-NO")]
    Norwegian,
    #[serde(rename = "pl-PL")]
    Polish,
    #[serde(rename = "pt-BR")]
    PortugueseBrazilian,
    #[serde(rename = "pt-PT")]
    PortugueseEuropean,
    #[serde(rename = "ro-RO")]
    Romanian,
    #[serde(rename = "ru-RU")]
    Russian,
    #[serde(rename = "es-ES")]
    SpanishSpain,
    #[serde(rename = "es-MX")]
    SpanishMexican,
    #[serde(rename = "es-US")]
    SpanishUS,
    #[serde(rename = "sv-SE")]
    Swedish,
    #[serde(rename = "tr-TR")]
    Turkish,
    #[serde(rename = "cy-GB")]
    Welsh,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum AwsAsrLanguage {
    #[serde(rename = "ab-GE")]
    Abkhaz,
    #[serde(rename = "af-ZA")]
    Afrikaans,
    #[serde(rename = "ar-AE")]
    ArabicGulf,
    #[serde(rename = "ar-SA")]
    ArabicModernStandard,
    #[serde(rename = "hy-AM")]
    Armenian,
    #[serde(rename = "ast-ES")]
    Asturian,
    #[serde(rename = "az-AZ")]
    Azerbaijani,
    #[serde(rename = "ba-RU")]
    Bashkir,
    #[serde(rename = "eu-ES")]
    Basque,
    #[serde(rename = "be-BY")]
    Belarusian,
    #[serde(rename = "bn-IN")]
    Bengali,
    #[serde(rename = "bs-BA")]
    Bosnian,
    #[serde(rename = "bg-BG")]
    Bulgarian,
    #[serde(rename = "ca-ES")]
    Catalan,
    #[serde(rename = "ckb-IR")]
    CentralKurdishIran,
    #[serde(rename = "ckb-IQ")]
    CentralKurdishIraq,
    #[serde(rename = "zh-HKyue-HK")]
    ChineseCantoneseHongKong,
    #[serde(rename = "zh-CN")]
    ChineseSimplified,
    #[serde(rename = "zh-TW")]
    ChineseTraditional,
    #[serde(rename = "hr-HR")]
    Croatian,
    #[serde(rename = "cs-CZ")]
    Czech,
    #[serde(rename = "da-DK")]
    Danish,
    #[serde(rename = "nl-NL")]
    Dutch,
    #[serde(rename = "en-AU")]
    EnglishAustralian,
    #[serde(rename = "en-GB")]
    EnglishBritish,
    #[serde(rename = "en-IN")]
    EnglishIndian,
    #[serde(rename = "en-IE")]
    EnglishIrish,
    #[serde(rename = "en-NZ")]
    EnglishNewZealand,
    #[serde(rename = "en-AB")]
    EnglishScottish,
    #[serde(rename = "en-ZA")]
    EnglishSouthAfrican,
    #[serde(rename = "en-US")]
    EnglishUS,
    #[serde(rename = "en-WL")]
    EnglishWelsh,
    #[serde(rename = "et-ET")]
    Estonian,
    #[serde(rename = "fa-IR")]
    Farsi,
    #[serde(rename = "fi-FI")]
    Finnish,
    #[serde(rename = "fr-FR")]
    French,
    #[serde(rename = "fr-CA")]
    FrenchCanadian,
    #[serde(rename = "gl-ES")]
    Galician,
    #[serde(rename = "ka-GE")]
    Georgian,
    #[serde(rename = "de-DE")]
    German,
    #[serde(rename = "de-CH")]
    GermanSwiss,
    #[serde(rename = "el-GR")]
    Greek,
    #[serde(rename = "gu-IN")]
    Gujarati,
    #[serde(rename = "ha-NG")]
    Hausa,
    #[serde(rename = "he-IL")]
    Hebrew,
    #[serde(rename = "hi-IN")]
    HindiIndian,
    #[serde(rename = "hu-HU")]
    Hungarian,
    #[serde(rename = "is-IS")]
    Icelandic,
    #[serde(rename = "id-ID")]
    Indonesian,
    #[serde(rename = "it-IT")]
    Italian,
    #[serde(rename = "ja-JP")]
    Japanese,
    #[serde(rename = "kab-DZ")]
    Kabyle,
    #[serde(rename = "kn-IN")]
    Kannada,
    #[serde(rename = "kk-KZ")]
    Kazakh,
    #[serde(rename = "rw-RW")]
    Kinyarwanda,
    #[serde(rename = "ko-KR")]
    Korean,
    #[serde(rename = "ky-KG")]
    Kyrgyz,
    #[serde(rename = "lv-LV")]
    Latvian,
    #[serde(rename = "lt-LT")]
    Lithuanian,
    #[serde(rename = "lg-IN")]
    Luganda,
    #[serde(rename = "mk-MK")]
    Macedonian,
    #[serde(rename = "ms-MY")]
    Malay,
    #[serde(rename = "ml-IN")]
    Malayalam,
    #[serde(rename = "mt-MT")]
    Maltese,
    #[serde(rename = "mr-IN")]
    Marathi,
    #[serde(rename = "mhr-RU")]
    MeadowMari,
    #[serde(rename = "mn-MN")]
    Mongolian,
    #[serde(rename = "no-NO")]
    NorwegianBokmål,
    #[serde(rename = "or-IN")]
    OdiaOrOriya,
    #[serde(rename = "ps-AF")]
    Pashto,
    #[serde(rename = "pl-PL")]
    Polish,
    #[serde(rename = "pt-PT")]
    Portuguese,
    #[serde(rename = "pt-BR")]
    PortugueseBrazilian,
    #[serde(rename = "pa-IN")]
    Punjabi,
    #[serde(rename = "ro-RO")]
    Romanian,
    #[serde(rename = "ru-RU")]
    Russian,
    #[serde(rename = "sr-RS")]
    Serbian,
    #[serde(rename = "si-LK")]
    Sinhala,
    #[serde(rename = "sk-SK")]
    Slovak,
    #[serde(rename = "sl-SI")]
    Slovenian,
    #[serde(rename = "so-SO")]
    Somali,
    #[serde(rename = "es-ES")]
    Spanish,
    #[serde(rename = "es-US")]
    SpanishUS,
    #[serde(rename = "su-ID")]
    Sundanese,
    #[serde(rename = "sw-KE")]
    SwahiliKenya,
    #[serde(rename = "sw-BI")]
    SwahiliBurundi,
    #[serde(rename = "sw-RW")]
    SwahiliRwanda,
    #[serde(rename = "sw-TZ")]
    SwahiliTanzania,
    #[serde(rename = "sw-UG")]
    SwahiliUganda,
    #[serde(rename = "sv-SE")]
    Swedish,
    #[serde(rename = "tl-PH")]
    TagalogOrFilipino,
    #[serde(rename = "ta-IN")]
    Tamil,
    #[serde(rename = "tt-RU")]
    Tatar,
    #[serde(rename = "te-IN")]
    Telugu,
    #[serde(rename = "th-TH")]
    Thai,
    #[serde(rename = "tr-TR")]
    Turkish,
    #[serde(rename = "uk-UA")]
    Ukrainian,
    #[serde(rename = "ug-CN")]
    Uyghur,
    #[serde(rename = "uz-UZ")]
    Uzbek,
    #[serde(rename = "vi-VN")]
    Vietnamese,
    #[serde(rename = "cy-WL")]
    Welsh,
    #[serde(rename = "wo-SN")]
    Wolof,
    #[serde(rename = "zu-ZA")]
    Zulu,
}
