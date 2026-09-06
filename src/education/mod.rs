#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
// SigmaOS Education Module
pub mod ncert_science_teacher;
pub mod outreach;
pub mod ncert_maths;

pub use ncert_science_teacher::{
    BloomsTaxonomyLevel, NcertChapterTopic, NcertGrade, NcertLessonPlan, NcertQuestionItem,
    NcertScienceTeacherSuite, NcertSubjectDomain, NcertVirtualLabExperiment,
};
pub use outreach::{
    DocAsset, DocFormat, EducationOutreachManager, LearningPath, UniversityPartnership,
};

pub use ncert_maths::{
    NcertClassGrade, NcertMathsDomain, NcertChapterSpec, NcertMathsFormulaRepository,
    Phase5E, NcertLessonPlanStep, NcertLessonPlan, NcertLessonPlanGenerator,
    NcertQuestionType, BloomsTaxonomyLevel, NcertQuestion, NcertQuestionBankManager,
    StepByStepSolution, NcertStepByStepSolutionSolver, StudentAssessmentEntry,
    NcertTeacherAnalyticsEngine,
};
