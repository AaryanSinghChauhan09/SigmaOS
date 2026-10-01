// SigmaOS Education Module
pub mod ncert_maths;
pub mod ncert_science_teacher;
pub mod outreach;

pub use ncert_science_teacher::{
    BloomsTaxonomyLevel, NcertChapterTopic, NcertGrade, NcertLessonPlan, NcertQuestionItem,
    NcertScienceTeacherSuite, NcertSubjectDomain, NcertVirtualLabExperiment,
};
pub use outreach::{
    DocAsset, DocFormat, EducationOutreachManager, LearningPath, UniversityPartnership,
};

pub use ncert_maths::{
    NcertChapterSpec, NcertClassGrade, NcertLessonPlanGenerator, NcertLessonPlanStep,
    NcertMathsDomain, NcertMathsFormulaRepository, NcertQuestion, NcertQuestionBankManager,
    NcertQuestionType, NcertStepByStepSolutionSolver, NcertTeacherAnalyticsEngine, Phase5E,
    StepByStepSolution, StudentAssessmentEntry,
};
