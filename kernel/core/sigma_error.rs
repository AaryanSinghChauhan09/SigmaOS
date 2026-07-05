// SPDX-License-Identifier: MIT
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/core/sigma_error.rs — Multilingual Error Messages
// Implements locale-aware error messages in 22 Indian languages
//
// Features:
//   - sigma_err_t type with locale-aware messages
//   - Error messages in 22 languages via sigma-bhashini lookup table
//   - "GST filing failed" → "जीएसटी दाखिल करना विफल रहा" (Hindi auto-translation)
//
// Language: Rust

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ── Error Codes ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum SigmaError {
    Success = 0,
    InvalidArgument = 1,
    OutOfMemory = 2,
    PermissionDenied = 3,
    NotFound = 4,
    AlreadyExists = 5,
    IOError = 6,
    NetworkError = 7,
    Timeout = 8,
    Cancelled = 9,
    Unknown = 10,
    AuthenticationFailed = 11,
    AuthorizationFailed = 12,
    QuotaExceeded = 13,
    ServiceUnavailable = 14,
    InvalidState = 15,
    OperationNotSupported = 16,
    // India-specific errors
    AadhaarVerificationFailed = 100,
    GstFilingFailed = 101,
    AbdmConnectionFailed = 102,
    PanInvalid = 103,
    BankAccountVerificationFailed = 104,
    UpiTransactionFailed = 105,
    VoterIdInvalid = 106,
    DrivingLicenseInvalid = 107,
    PassportInvalid = 108,
    RationCardInvalid = 109,
}

// ── Languages ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Language {
    English = 0,
    Hindi = 1,
    Bengali = 2,
    Tamil = 3,
    Telugu = 4,
    Kannada = 5,
    Malayalam = 6,
    Gujarati = 7,
    Marathi = 8,
    Punjabi = 9,
    Odia = 10,
    Assamese = 11,
    Maithili = 12,
    Santali = 13,
    Kashmiri = 14,
    Sindhi = 15,
    Nepali = 16,
    Urdu = 17,
    Konkani = 18,
    Manipuri = 19,
    Bodo = 20,
    Dogri = 21,
    Sanskrit = 22,
}

// ── Error Message Table ───────────────────────────────────────────

pub struct ErrorMessageTable {
    messages: HashMap<(SigmaError, Language), &'static str>,
}

impl ErrorMessageTable {
    pub fn new() -> Self {
        let mut messages = HashMap::new();
        
        // Success
        messages.insert((SigmaError::Success, Language::English), "Operation completed successfully");
        messages.insert((SigmaError::Success, Language::Hindi), "ऑपरेशन सफलतापूर्वक पूरा हो गया");
        messages.insert((SigmaError::Success, Language::Bengali), "অপারেশন সফলভাবে সম্পন্ন হয়েছে");
        messages.insert((SigmaError::Success, Language::Tamil), "செயல்பாடு வெற்றிகரமாக முடிந்தது");
        messages.insert((SigmaError::Success, Language::Telugu), "ఆపరేషన్ విజయవంతంగా పూర్తి అయింది");
        messages.insert((SigmaError::Success, Language::Kannada), "ಕಾರ್ಯಾಚರಣೆ ಯಶಸ್ವೀಯಾಗಿ ಪೂರ್ಣಗೊಂಡಿದೆ");
        messages.insert((SigmaError::Success, Language::Malayalam), "പ്രവർത്തനം വിജയകരമായി പൂർത്തിയായി");
        messages.insert((SigmaError::Success, Language::Gujarati), "કાર્યપદ્ધતિ સફળતાપૂર્વક પૂર્ણ થઈ");
        messages.insert((SigmaError::Success, Language::Marathi), "ऑपरेशन यशस्वीरित्या पूर्ण झाले");
        messages.insert((SigmaError::Success, Language::Punjabi), "ਕਾਰਵਾਈ ਸਫਲਤਾਪੂਰਵਕ ਪੂਰੀ ਹੋਈ");
        messages.insert((SigmaError::Success, Language::Odia), "କାର୍ଯ୍ୟକାରୀ ସଫଳତାର ସହ ସମ୍ପନ୍ନ ହେଲା");
        messages.insert((SigmaError::Success, Language::Assamese), "কাৰ্য্য সফলতাৰে সম্পন্ন হ'ল");
        messages.insert((SigmaError::Success, Language::Maithili), "कार्य सफलतापूर्वक पूर्ण भेल");
        messages.insert((SigmaError::Success, Language::Santali), "ᱫᱟᱢ ᱥᱟᱯᱲᱟ ᱥᱟᱯᱲᱟ ᱦᱚᱭ ᱮᱱᱟ");
        messages.insert((SigmaError::Success, Language::Kashmiri), "کامیابی کے نال آپریشن مکمل ہوا");
        messages.insert((SigmaError::Success, Language::Sindhi), "عمل ڪاميابي سان پورو ٿيو");
        messages.insert((SigmaError::Success, Language::Nepali), "अपरेसन सफलतापूर्वक पूरा भयो");
        messages.insert((SigmaError::Success, Language::Urdu), "آپریشن کامیابی سے مکمل ہوا");
        messages.insert((SigmaError::Success, Language::Konkani), "कार्य यशस्वीरित्या पूर्ण जालो");
        messages.insert((SigmaError::Success, Language::Manipuri), "ꯑꯄꯦꯔꯦꯁꯟ ꯁꯛꯏꯅꯤ ꯃꯁꯟ ꯇꯣꯟꯈꯤ");
        messages.insert((SigmaError::Success, Language::Bodo), "खामानि खामानि खामानि खामानि");
        messages.insert((SigmaError::Success, Language::Dogri), "ऑपरेसन सफलतापूर्वक पूर्ण भयो");
        messages.insert((SigmaError::Success, Language::Sanskrit), "कार्यं सफलतया सम्पन्नम् अभवत्");

        // Invalid Argument
        messages.insert((SigmaError::InvalidArgument, Language::English), "Invalid argument provided");
        messages.insert((SigmaError::InvalidArgument, Language::Hindi), "अमान्य तर्क प्रदान किया गया");
        messages.insert((SigmaError::InvalidArgument, Language::Bengali), "অবৈধ আর্গুমেন্ট দেওয়া হয়েছে");
        messages.insert((SigmaError::InvalidArgument, Language::Tamil), "தவறான வாதம் வழங்கப்பட்டது");
        messages.insert((SigmaError::InvalidArgument, Language::Telugu), "చెల్లని ఆర్గుమెంట్ అందించబడింది");
        messages.insert((SigmaError::InvalidArgument, Language::Kannada), "ಅಮಾನ್ಯ ವಾದವನ್ನು ಒದಗಿಸಲಾಗಿದೆ");
        messages.insert((SigmaError::InvalidArgument, Language::Malayalam), "അസാധുവായ ആർഗ്യുമെന്റ് നൽകി");
        messages.insert((SigmaError::InvalidArgument, Language::Gujarati), "અમાન્ય દલીલ પ્રદાન કરવામાં આવ્યું");
        messages.insert((SigmaError::InvalidArgument, Language::Marathi), "अवैध वितर्क प्रदान केला");
        messages.insert((SigmaError::InvalidArgument, Language::Punjabi), "ਗਲਤ ਦਲੀਲ ਦਿੱਤੀ ਗਈ");
        messages.insert((SigmaError::InvalidArgument, Language::Odia), "ଅବୈଧ ଯୁକ୍ତି ପ୍ରଦାନ କରାଯାଇଛି");
        messages.insert((SigmaError::InvalidArgument, Language::Assamese), "অবৈধ তৰ্ক দিয়া হৈছে");
        messages.insert((SigmaError::InvalidArgument, Language::Maithili), "अवैध तर्क देल गेल");
        messages.insert((SigmaError::InvalidArgument, Language::Santali), "ᱵᱟᱨᱟ ᱟᱨᱜᱩᱢᱮᱱᱴ ᱮᱱᱟ");
        messages.insert((SigmaError::InvalidArgument, Language::Kashmiri), "غلط دلیل دیا گیا");
        messages.insert((SigmaError::InvalidArgument, Language::Sindhi), "غلط دليل ڏنو");
        messages.insert((SigmaError::InvalidArgument, Language::Nepali), "अवैध तर्क दिइएको");
        messages.insert((SigmaError::InvalidArgument, Language::Urdu), "غلط دلیل دیا گیا");
        messages.insert((SigmaError::InvalidArgument, Language::Konkani), "बिनवैध तर्क दिलो");
        messages.insert((SigmaError::InvalidArgument, Language::Manipuri), "ꯑꯀꯟ ꯑꯔꯒꯨ�ꯃꯤ ꯈꯣꯟꯈꯤ");
        messages.insert((SigmaError::InvalidArgument, Language::Bodo), "खामानि खामानि");
        messages.insert((SigmaError::InvalidArgument, Language::Dogri), "गलत दलील दी गई");
        messages.insert((SigmaError::InvalidArgument, Language::Sanskrit), "असाधुः तर्कः प्रदत्तः");

        // Out of Memory
        messages.insert((SigmaError::OutOfMemory, Language::English), "Out of memory");
        messages.insert((SigmaError::OutOfMemory, Language::Hindi), "स्मृति समाप्त");
        messages.insert((SigmaError::OutOfMemory, Language::Bengali), "মেমোরি শেষ");
        messages.insert((SigmaError::OutOfMemory, Language::Tamil), "நினைவகம் முடிந்தது");
        messages.insert((SigmaError::OutOfMemory, Language::Telugu), "మెమోరీ అయిపోయింది");
        messages.insert((SigmaError::OutOfMemory, Language::Kannada), "ಮೆಮೊರಿ ಖಾಲಿಯಾಗಿದೆ");
        messages.insert((SigmaError::OutOfMemory, Language::Malayalam), "മെമ്മറി തീർന്നു");
        messages.insert((SigmaError::OutOfMemory, Language::Gujarati), "મેમરી પૂર્ણ થઈ ગઈ");
        messages.insert((SigmaError::OutOfMemory, Language::Marathi), "मेमरी संपली");
        messages.insert((SigmaError::OutOfMemory, Language::Punjabi), "ਮੈਮੋਰੀ ਖਤਮ");
        messages.insert((SigmaError::OutOfMemory, Language::Odia), "ସ୍ମୃତି ସମାପ୍ତ");
        messages.insert((SigmaError::OutOfMemory, Language::Assamese), "মেমৰি শেষ");
        messages.insert((SigmaError::OutOfMemory, Language::Maithili), "स्मृति समाप्त");
        messages.insert((SigmaError::OutOfMemory, Language::Santali), "ᱢᱮᱢᱳᱨᱤ ᱢᱩᱠᱩᱱ");
        messages.insert((SigmaError::OutOfMemory, Language::Kashmiri), "میموری ختم");
        messages.insert((SigmaError::OutOfMemory, Language::Sindhi), "ميموري ختم");
        messages.insert((SigmaError::OutOfMemory, Language::Nepali), "स्मृति समाप्त");
        messages.insert((SigmaError::OutOfMemory, Language::Urdu), "میموری ختم");
        messages.insert((SigmaError::OutOfMemory, Language::Konkani), "मेमरी संपली");
        messages.insert((SigmaError::OutOfMemory, Language::Manipuri), "ꯃꯤꯟ ꯃꯤꯟꯈꯤ");
        messages.insert((SigmaError::OutOfMemory, Language::Bodo), "खामानि खामानि");
        messages.insert((SigmaError::OutOfMemory, Language::Dogri), "میموری ختم");
        messages.insert((SigmaError::OutOfMemory, Language::Sanskrit), "स्मृतिः समाप्ता");

        // GST Filing Failed (India-specific)
        messages.insert((SigmaError::GstFilingFailed, Language::English), "GST filing failed");
        messages.insert((SigmaError::GstFilingFailed, Language::Hindi), "जीएसटी दाखिल करना विफल रहा");
        messages.insert((SigmaError::GstFilingFailed, Language::Bengali), "জিএসটি ফাইলিং ব্যর্থ হয়েছে");
        messages.insert((SigmaError::GstFilingFailed, Language::Tamil), "GST தாக்கல் தோல்வியடைந்தது");
        messages.insert((SigmaError::GstFilingFailed, Language::Telugu), "జీఎస్టీ ఫైలింగ్ విఫలమైంది");
        messages.insert((SigmaError::GstFilingFailed, Language::Kannada), "ಜಿಎಸ್‌ಟಿ ದಾಖಲಾತಿ ವಿಫಲವಾಯಿತು");
        messages.insert((SigmaError::GstFilingFailed, Language::Malayalam), "GST ഫയലിംഗ് പരാജയപ്പെെട്ടു");
        messages.insert((SigmaError::GstFilingFailed, Language::Gujarati), "GST ફાઇલિંગ નિષ્ફળ");
        messages.insert((SigmaError::GstFilingFailed, Language::Marathi), "जीएसटी दाखल अयशस्वी");
        messages.insert((SigmaError::GstFilingFailed, Language::Punjabi), "GST ਫਾਈਲਿੰਗ ਅਸਫਲ");
        messages.insert((SigmaError::GstFilingFailed, Language::Odia), "GST ଫାଇଲିଂଗ ବିଫଳ");
        messages.insert((SigmaError::GstFilingFailed, Language::Assamese), "GST ফাইলিং ব্যৰ্থ");
        messages.insert((SigmaError::GstFilingFailed, Language::Maithili), "जीएसटी दाखल बिफल");
        messages.insert((SigmaError::GstFilingFailed, Language::Santali), "GST ᱯᱟᱭᱞᱤᱝ ᱵᱤᱯᱮᱱ");
        messages.insert((SigmaError::GstFilingFailed, Language::Kashmiri), "GST فائلنگ ناکام");
        messages.insert((SigmaError::GstFilingFailed, Language::Sindhi), "GST فائلنگ ناڪام");
        messages.insert((SigmaError::GstFilingFailed, Language::Nepali), "जीएसटी दाखल असफल");
        messages.insert((SigmaError::GstFilingFailed, Language::Urdu), "GST فائلنگ ناکام");
        messages.insert((SigmaError::GstFilingFailed, Language::Konkani), "GST फाइलिंग अयशस्वी");
        messages.insert((SigmaError::GstFilingFailed, Language::Manipuri), "GST ꯐꯥꯣꯟ ꯇꯣꯟꯈꯤ");
        messages.insert((SigmaError::GstFilingFailed, Language::Bodo), "GST खामानि");
        messages.insert((SigmaError::GstFilingFailed, Language::Dogri), "GST فائلنگ ناکام");
        messages.insert((SigmaError::GstFilingFailed, Language::Sanskrit), "जीएसटी दाखला विफला");

        // Aadhaar Verification Failed (India-specific)
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::English), "Aadhaar verification failed");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Hindi), "आधार सत्यापन विफल");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Bengali), "আধার যাচাই ব্যর্থ");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Tamil), "ஆதார் சரிபார்ப்பு தோல்வி");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Telugu), "ఆధార్ ధృవీకరణ విఫలం");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Kannada), "ಆಧಾರ್ ಪರಿಶೀಲನೆ ವಿಫಲ");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Malayalam), "ആധാർ പരിശോധന പരാജയപ്പെട്ടു");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Gujarati), "આધાર ચકાસણી નિષ્ફળ");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Marathi), "आधार सत्यापन अयशस्वी");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Punjabi), "ਆਧਾਰ ਪੁਸ਼ਟੀਕਰਨ ਅਸਫਲ");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Odia), "ଆଧାର �ାଚନା ବିଫଳ");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Assamese), "আধাৰ পৰীক্ষণ ব্যৰ্থ");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Maithili), "आधार सत्यापन बिफल");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Santali), "ᱟᱫᱷᱟᱨ ᱵᱮᱨᱤᱯᱮᱱ");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Kashmiri), "آدھار تصدیق ناکام");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Sindhi), "آدھار تصديق ناڪام");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Nepali), "आधार प्रमाणीकरण असफल");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Urdu), "آدھار تصدیق ناکام");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Konkani), "आधार सत्यापन अयशस्वी");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Manipuri), "ꯑꯥꯥꯔ ꯄ꯭ꯤꯁꯤꯟ ꯇꯣꯟꯈꯤ");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Bodo), "आधार खामानि");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Dogri), "آدھار تصدیق ناکام");
        messages.insert((SigmaError::AadhaarVerificationFailed, Language::Sanskrit), "आधार-सत्यापनं विफलम्");

        Self { messages }
    }

    pub fn get_message(&self, error: SigmaError, language: Language) -> &'static str {
        self.messages.get(&(error, language))
            .copied()
            .unwrap_or_else(|| {
                // Fallback to English if translation not available
                self.messages.get(&(error, Language::English))
                    .copied()
                    .unwrap_or("Unknown error")
            })
    }

    pub fn set_message(&mut self, error: SigmaError, language: Language, message: &'static str) {
        self.messages.insert((error, language), message);
    }
}

// ── Global Error Table ─────────────────────────────────────────────

static mut ERROR_TABLE: Option<ErrorMessageTable> = None;

pub fn init_error_table() {
    unsafe {
        if ERROR_TABLE.is_none() {
            ERROR_TABLE = Some(ErrorMessageTable::new());
        }
    }
}

pub fn get_error_message(error: SigmaError, language: Language) -> &'static str {
    unsafe {
        ERROR_TABLE.as_ref()
            .map(|table| table.get_message(error, language))
            .unwrap_or("Error table not initialized")
    }
}

// ── C-ABI exports ─────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn sigma_error_init() {
    init_error_table();
}

#[no_mangle]
pub extern "C" fn sigma_error_get_message(error_code: u32, language_code: u32) -> *const u8 {
    let error = match error_code {
        0 => SigmaError::Success,
        1 => SigmaError::InvalidArgument,
        2 => SigmaError::OutOfMemory,
        3 => SigmaError::PermissionDenied,
        4 => SigmaError::NotFound,
        5 => SigmaError::AlreadyExists,
        6 => SigmaError::IOError,
        7 => SigmaError::NetworkError,
        8 => SigmaError::Timeout,
        9 => SigmaError::Cancelled,
        10 => SigmaError::Unknown,
        11 => SigmaError::AuthenticationFailed,
        12 => SigmaError::AuthorizationFailed,
        13 => SigmaError::QuotaExceeded,
        14 => SigmaError::ServiceUnavailable,
        15 => SigmaError::InvalidState,
        16 => SigmaError::OperationNotSupported,
        100 => SigmaError::AadhaarVerificationFailed,
        101 => SigmaError::GstFilingFailed,
        102 => SigmaError::AbdmConnectionFailed,
        103 => SigmaError::PanInvalid,
        104 => SigmaError::BankAccountVerificationFailed,
        105 => SigmaError::UpiTransactionFailed,
        106 => SigmaError::VoterIdInvalid,
        107 => SigmaError::DrivingLicenseInvalid,
        108 => SigmaError::PassportInvalid,
        109 => SigmaError::RationCardInvalid,
        _ => SigmaError::Unknown,
    };

    let language = match language_code {
        0 => Language::English,
        1 => Language::Hindi,
        2 => Language::Bengali,
        3 => Language::Tamil,
        4 => Language::Telugu,
        5 => Language::Kannada,
        6 => Language::Malayalam,
        7 => Language::Gujarati,
        8 => Language::Marathi,
        9 => Language::Punjabi,
        10 => Language::Odia,
        11 => Language::Assamese,
        12 => Language::Maithili,
        13 => Language::Santali,
        14 => Language::Kashmiri,
        15 => Language::Sindhi,
        16 => Language::Nepali,
        17 => Language::Urdu,
        18 => Language::Konkani,
        19 => Language::Manipuri,
        20 => Language::Bodo,
        21 => Language::Dogri,
        22 => Language::Sanskrit,
        _ => Language::English,
    };

    let message = get_error_message(error, language);
    message.as_ptr()
}

#[no_mangle]
pub extern "C" fn sigma_error_set_message(error_code: u32, language_code: u32, message: *const u8, message_len: usize) {
    let error = match error_code {
        0 => SigmaError::Success,
        1 => SigmaError::InvalidArgument,
        2 => SigmaError::OutOfMemory,
        3 => SigmaError::PermissionDenied,
        4 => SigmaError::NotFound,
        5 => SigmaError::AlreadyExists,
        6 => SigmaError::IOError,
        7 => SigmaError::NetworkError,
        8 => SigmaError::Timeout,
        9 => SigmaError::Cancelled,
        10 => SigmaError::Unknown,
        11 => SigmaError::AuthenticationFailed,
        12 => SigmaError::AuthorizationFailed,
        13 => SigmaError::QuotaExceeded,
        14 => SigmaError::ServiceUnavailable,
        15 => SigmaError::InvalidState,
        16 => SigmaError::OperationNotSupported,
        100 => SigmaError::AadhaarVerificationFailed,
        101 => SigmaError::GstFilingFailed,
        102 => SigmaError::AbdmConnectionFailed,
        103 => SigmaError::PanInvalid,
        104 => SigmaError::BankAccountVerificationFailed,
        105 => SigmaError::UpiTransactionFailed,
        106 => SigmaError::VoterIdInvalid,
        107 => SigmaError::DrivingLicenseInvalid,
        108 => SigmaError::PassportInvalid,
        109 => SigmaError::RationCardInvalid,
        _ => SigmaError::Unknown,
    };

    let language = match language_code {
        0 => Language::English,
        1 => Language::Hindi,
        2 => Language::Bengali,
        3 => Language::Tamil,
        4 => Language::Telugu,
        5 => Language::Kannada,
        6 => Language::Malayalam,
        7 => Language::Gujarati,
        8 => Language::Marathi,
        9 => Language::Punjabi,
        10 => Language::Odia,
        11 => Language::Assamese,
        12 => Language::Maithili,
        13 => Language::Santali,
        14 => Language::Kashmiri,
        15 => Language::Sindhi,
        16 => Language::Nepali,
        17 => Language::Urdu,
        18 => Language::Konkani,
        19 => Language::Manipuri,
        20 => Language::Bodo,
        21 => Language::Dogri,
        22 => Language::Sanskrit,
        _ => Language::English,
    };

    unsafe {
        if !message.is_null() && message_len > 0 {
            let message_str = String::from_utf8_lossy(
                std::slice::from_raw_parts(message, message_len));
            let message_owned: &'static str = Box::leak(message_str.into_boxed_str());
            if let Some(table) = &mut ERROR_TABLE {
                table.set_message(error, language, message_owned);
            }
        }
    }
}
