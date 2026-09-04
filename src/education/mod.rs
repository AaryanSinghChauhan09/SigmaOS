// SigmaOS Education Module
pub mod ncert_science_teacher;
pub mod outreach;

pub use ncert_science_teacher::{
    BloomsTaxonomyLevel, NcertChapterTopic, NcertGrade, NcertLessonPlan, NcertQuestionItem,
    NcertScienceTeacherSuite, NcertSubjectDomain, NcertVirtualLabExperiment,
};
pub use outreach::{
    DocAsset, DocFormat, EducationOutreachManager, LearningPath, UniversityPartnership,
};
