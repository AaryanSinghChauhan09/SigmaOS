//! Office suite integration for SigmaOS.
//!
//! Covers: BetterOffice, CraftOffice, GenOffice, OnlyOffice, and
//! LibreOffice/OpenOffice module integration.
//!
//! SigmaOS-office is an offline Chromium web-browser-based office suite that
//! aggregates: OpenOffice, LibreOffice, GenOffice, OnlyOffice, CraftOffice.

#![allow(dead_code)]

/// Office suite type available in SigmaOS.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfficeSuite {
    /// LibreOffice — full FOSS suite (Writer, Calc, Impress, Draw, Math, Base).
    LibreOffice,
    /// Apache OpenOffice — legacy FOIS suite.
    OpenOffice,
    /// OnlyOffice — web-based, MS Office compatible, self-hosted.
    OnlyOffice,
    /// BetterOffice — SigmaOS lightweight document editor.
    BetterOffice,
    /// CraftOffice — creative document suite (3D/Ani/Art integration).
    CraftOffice,
    /// GenOffice — general-purpose office (Docs, Sheet, Slides, PDF, Markdown, HTML).
    GenOffice,
}

impl OfficeSuite {
    pub fn name(self) -> &'static str {
        match self {
            Self::LibreOffice  => "LibreOffice",
            Self::OpenOffice   => "Apache OpenOffice",
            Self::OnlyOffice   => "OnlyOffice",
            Self::BetterOffice => "BetterOffice",
            Self::CraftOffice  => "CraftOffice",
            Self::GenOffice    => "GenOffice",
        }
    }

    /// Whether this suite is the SigmaOS default.
    pub fn is_default(self) -> bool {
        matches!(self, Self::LibreOffice)
    }

    /// Whether this suite works offline (no network required).
    pub fn is_offline(self) -> bool {
        matches!(self, Self::LibreOffice | Self::OpenOffice | Self::BetterOffice | Self::CraftOffice | Self::GenOffice)
    }

    /// Whether it uses a Chromium-based rendering engine.
    pub fn is_chromium_based(self) -> bool {
        matches!(self, Self::OnlyOffice | Self::GenOffice | Self::CraftOffice | Self::BetterOffice)
    }

    /// Modules (applications) included in this suite.
    pub fn modules(self) -> Vec<OfficeModule> {
        match self {
            Self::LibreOffice => vec![
                OfficeModule::Writer, OfficeModule::Calc, OfficeModule::Impress,
                OfficeModule::Draw,  OfficeModule::Math,  OfficeModule::Base,
            ],
            Self::OpenOffice => vec![
                OfficeModule::Writer, OfficeModule::Calc, OfficeModule::Impress,
                OfficeModule::Draw,  OfficeModule::Math,  OfficeModule::Base,
            ],
            Self::OnlyOffice => vec![
                OfficeModule::Documents, OfficeModule::Spreadsheets,
                OfficeModule::Presentations, OfficeModule::Pdf,
            ],
            Self::BetterOffice => vec![
                OfficeModule::Documents, OfficeModule::Spreadsheets,
                OfficeModule::Diagrams,
            ],
            Self::CraftOffice => vec![
                OfficeModule::Documents, OfficeModule::Spreadsheets,
                OfficeModule::Slides, OfficeModule::Diagrams,
            ],
            Self::GenOffice => vec![
                OfficeModule::Documents, OfficeModule::Spreadsheets,
                OfficeModule::Slides, OfficeModule::Pdf,
                OfficeModule::Markdown, OfficeModule::Html,
            ],
        }
    }

    /// Supported file format extensions.
    pub fn file_formats(self) -> Vec<&'static str> {
        match self {
            Self::LibreOffice | Self::OpenOffice => vec![
                "odt", "ods", "odp", "odg", "docx", "xlsx", "pptx", "pdf", "csv", "rtf",
            ],
            Self::OnlyOffice => vec![
                "docx", "xlsx", "pptx", "odt", "ods", "odp", "pdf", "csv", "rtf",
            ],
            Self::BetterOffice => vec!["docx", "xlsx", "pdf", "md", "html"],
            Self::CraftOffice => vec!["docx", "xlsx", "pptx", "pdf", "svg", "md"],
            Self::GenOffice => vec!["docx", "xlsx", "pptx", "pdf", "md", "html", "csv"],
        }
    }
}

/// Individual application module within an office suite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OfficeModule {
    /// Word processor (LibreOffice Writer).
    Writer,
    /// Spreadsheet (LibreOffice Calc).
    Calc,
    /// Presentation (LibreOffice Impress).
    Impress,
    /// Drawing / vector graphics (LibreOffice Draw).
    Draw,
    /// Formula editor (LibreOffice Math).
    Math,
    /// Database manager (LibreOffice Base).
    Base,
    /// Generic document editor.
    Documents,
    /// Generic spreadsheet.
    Spreadsheets,
    /// Generic presentation.
    Slides,
    /// PDF viewer/editor.
    Pdf,
    /// Diagram editor.
    Diagrams,
    /// Markdown editor.
    Markdown,
    /// HTML editor.
    Html,
}

impl OfficeModule {
    pub fn name(self) -> &'static str {
        match self {
            Self::Writer       => "Writer",
            Self::Calc         => "Calc",
            Self::Impress      => "Impress",
            Self::Draw         => "Draw",
            Self::Math         => "Math",
            Self::Base         => "Base",
            Self::Documents    => "Documents",
            Self::Spreadsheets  => "Spreadsheets",
            Self::Slides       => "Slides",
            Self::Pdf          => "PDF",
            Self::Diagrams     => "Diagrams",
            Self::Markdown     => "Markdown",
            Self::Html         => "HTML",
        }
    }
}

/// MIME type mapping for office file associations.
pub fn office_mime_types() -> Vec<(&'static str, OfficeModule)> {
    vec![
        ("application/vnd.oasis.opendocument.text",        OfficeModule::Writer),
        ("application/vnd.oasis.opendocument.spreadsheet",  OfficeModule::Calc),
        ("application/vnd.oasis.opendocument.presentation", OfficeModule::Impress),
        ("application/vnd.oasis.opendocument.graphics",     OfficeModule::Draw),
        ("application/msword",                               OfficeModule::Writer),
        ("application/vnd.openxmlformats-officedocument.wordprocessingml.document", OfficeModule::Writer),
        ("application/vnd.ms-excel",                         OfficeModule::Calc),
        ("application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",        OfficeModule::Calc),
        ("application/vnd.ms-powerpoint",                   OfficeModule::Impress),
        ("application/vnd.openxmlformats-officedocument.presentationml.presentation", OfficeModule::Impress),
        ("application/pdf",                                  OfficeModule::Pdf),
        ("text/markdown",                                    OfficeModule::Markdown),
        ("text/html",                                       OfficeModule::Html),
    ]
}

/// All office suites available in SigmaOS.
pub const ALL_OFFICE_SUITES: &[OfficeSuite] = &[
    OfficeSuite::LibreOffice,
    OfficeSuite::OpenOffice,
    OfficeSuite::OnlyOffice,
    OfficeSuite::BetterOffice,
    OfficeSuite::CraftOffice,
    OfficeSuite::GenOffice,
];

/// The SigmaOS-office architecture: Chromium web browser running office apps offline.
#[derive(Debug, Clone)]
pub struct SigmaOfficeConfig {
    /// Which suite is the default for new documents.
    pub default_suite: OfficeSuite,
    /// Whether to install OnlyOffice (requires Chromium runtime).
    pub install_onlyoffice: bool,
    /// Whether to install LibreOffice (full desktop integration).
    pub install_libreoffice: bool,
    /// Whether to install GenOffice (lightweight Markdown/HTML editor).
    pub install_genoffice: bool,
}

impl SigmaOfficeConfig {
    /// V1 default: LibreOffice as primary, no extras.
    pub fn v1_default() -> Self {
        Self {
            default_suite: OfficeSuite::LibreOffice,
            install_onlyoffice: false,
            install_libreoffice: true,
            install_genoffice: false,
        }
    }

    /// Full install with all suites.
    pub fn full() -> Self {
        Self {
            default_suite: OfficeSuite::LibreOffice,
            install_onlyoffice: true,
            install_libreoffice: true,
            install_genoffice: true,
        }
    }

    /// List of suites that will be installed with this config.
    pub fn installed_suites(&self) -> Vec<OfficeSuite> {
        let mut suites = Vec::new();
        if self.install_libreoffice  { suites.push(OfficeSuite::LibreOffice); }
        if self.install_onlyoffice   { suites.push(OfficeSuite::OnlyOffice); }
        if self.install_genoffice    { suites.push(OfficeSuite::GenOffice); }
        suites
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_six_suites_present() {
        assert_eq!(ALL_OFFICE_SUITES.len(), 6);
    }

    #[test]
    fn test_libreoffice_has_six_modules() {
        let mods = OfficeSuite::LibreOffice.modules();
        assert_eq!(mods.len(), 6);
        assert!(mods.contains(&OfficeModule::Base));
    }

    #[test]
    fn test_genoffice_includes_markdown_and_html() {
        let mods = OfficeSuite::GenOffice.modules();
        assert!(mods.contains(&OfficeModule::Markdown));
        assert!(mods.contains(&OfficeModule::Html));
    }

    #[test]
    fn test_v1_default_installs_only_libreoffice() {
        let cfg = SigmaOfficeConfig::v1_default();
        let installed = cfg.installed_suites();
        assert_eq!(installed.len(), 1);
        assert!(installed.contains(&OfficeSuite::LibreOffice));
    }

    #[test]
    fn test_libreoffice_is_offline() {
        assert!(OfficeSuite::LibreOffice.is_offline());
    }

    #[test]
    fn test_onlyoffice_is_chromium_based() {
        assert!(OfficeSuite::OnlyOffice.is_chromium_based());
    }

    #[test]
    fn test_office_mime_types_nonempty() {
        assert!(!office_mime_types().is_empty());
    }
}
