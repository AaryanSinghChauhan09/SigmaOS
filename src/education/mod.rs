// SigmaOS Education Module
pub mod outreach;
pub mod ncert_maths;

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
