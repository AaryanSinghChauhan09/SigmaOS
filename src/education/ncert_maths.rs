#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

// ============================================================================
// 📐 1. NCERT CURRICULUM CLASSIFICATION & DOMAINS
// ============================================================================

/// NCERT Mathematics Class Grades (Class 6 through Class 12)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum NcertClassGrade {
    Class6 = 6,
    Class7 = 7,
    Class8 = 8,
    Class9 = 9,
    Class10 = 10,
    Class11 = 11,
    Class12 = 12,
}

/// NCERT Mathematics Subject Domains
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NcertMathsDomain {
    NumberSystems,
    Algebra,
    Geometry,
    Mensuration,
    Trigonometry,
    CoordinateGeometry,
    Calculus,
    MatricesAndDeterminants,
    VectorsAnd3D,
    ProbabilityAndStatistics,
    LinearProgramming,
}

/// Chapter specification according to NCERT syllabus
#[derive(Debug, Clone)]
pub struct NcertChapterSpec {
    pub class_grade: NcertClassGrade,
    pub chapter_number: u8,
    pub chapter_title: String,
    pub domain: NcertMathsDomain,
    pub estimated_teaching_hours: u8,
    pub key_formulas: Vec<String>,
}

// ============================================================================
// 📐 2. FORMULA REPOSITORY & BLACKBOARD LATEX RENDERER
// ============================================================================

/// NCERT Formula and Blackboard LaTeX renderer for Smart Boards
pub struct NcertMathsFormulaRepository {
    pub chapters: BTreeMap<(u8, u8), NcertChapterSpec>, // (grade_num, chapter_num) -> Spec
}

impl NcertMathsFormulaRepository {
    pub fn new() -> Self {
        let mut repo = Self {
            chapters: BTreeMap::new(),
        };
        repo.populate_standard_ncert_chapters();
        repo
    }

    fn populate_standard_ncert_chapters(&mut self) {
        // Class 10 Chapter 2: Polynomials
        self.chapters.insert((10, 2), NcertChapterSpec {
            class_grade: NcertClassGrade::Class10,
            chapter_number: 2,
            chapter_title: "Polynomials".to_string(),
            domain: NcertMathsDomain::Algebra,
            estimated_teaching_hours: 8,
            key_formulas: vec![
                "\\alpha + \\beta = -\\frac{b}{a}".to_string(),
                "\\alpha \\beta = \\frac{c}{a}".to_string(),
            ],
        });

        // Class 10 Chapter 8: Introduction to Trigonometry
        self.chapters.insert((10, 8), NcertChapterSpec {
            class_grade: NcertClassGrade::Class10,
            chapter_number: 8,
            chapter_title: "Introduction to Trigonometry".to_string(),
            domain: NcertMathsDomain::Trigonometry,
            estimated_teaching_hours: 10,
            key_formulas: vec![
                "\\sin^2 \\theta + \\cos^2 \\theta = 1".to_string(),
                "1 + \\tan^2 \\theta = \\sec^2 \\theta".to_string(),
                "1 + \\cot^2 \\theta = \\csc^2 \\theta".to_string(),
            ],
        });

        // Class 12 Chapter 3: Matrices
        self.chapters.insert((12, 3), NcertChapterSpec {
            class_grade: NcertClassGrade::Class12,
            chapter_number: 3,
            chapter_title: "Matrices".to_string(),
            domain: NcertMathsDomain::MatricesAndDeterminants,
            estimated_teaching_hours: 12,
            key_formulas: vec![
                "A \\cdot A^{-1} = I".to_string(),
                "(AB)^T = B^T A^T".to_string(),
            ],
        });

        // Class 12 Chapter 7: Integrals
        self.chapters.insert((12, 7), NcertChapterSpec {
            class_grade: NcertClassGrade::Class12,
            chapter_number: 7,
            chapter_title: "Integrals".to_string(),
            domain: NcertMathsDomain::Calculus,
            estimated_teaching_hours: 20,
            key_formulas: vec![
                "\\int x^n dx = \\frac{x^{n+1}}{n+1} + C".to_string(),
                "\\int e^x dx = e^x + C".to_string(),
                "\\int \\frac{1}{x} dx = \\ln|x| + C".to_string(),
            ],
        });
    }

    pub fn get_chapter(&self, grade: u8, chapter: u8) -> Option<&NcertChapterSpec> {
        self.chapters.get(&(grade, chapter))
    }

    pub fn render_blackboard_latex(&self, grade: u8, chapter: u8) -> String {
        if let Some(spec) = self.get_chapter(grade, chapter) {
            let mut latex = format!("\\section*{{Class {} Ch {}: {}}}\n\\begin{{align*}}\n", spec.class_grade as u8, spec.chapter_number, spec.chapter_title);
            for formula in &spec.key_formulas {
                latex.push_str(&format!("  {} \\\\\n", formula));
            }
            latex.push_str("\\end{align*}\n");
            latex
        } else {
            "\\text{Chapter not found in NCERT repository}".to_string()
        }
    }
}

impl Default for NcertMathsFormulaRepository {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 📐 3. LESSON PLAN GENERATOR (5E INSTRUCTIONAL MODEL)
// ============================================================================

/// 5E Model Phases for Pedagogy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Phase5E {
    Engage,
    Explore,
    Explain,
    Elaborate,
    Evaluate,
}

#[derive(Debug, Clone)]
pub struct NcertLessonPlanStep {
    pub phase: Phase5E,
    pub duration_minutes: u16,
    pub teacher_activity: String,
    pub student_activity: String,
    pub blackboard_teaching_aids: String,
}

#[derive(Debug, Clone)]
pub struct NcertLessonPlan {
    pub topic: String,
    pub grade: NcertClassGrade,
    pub learning_objectives: Vec<String>,
    pub steps: Vec<NcertLessonPlanStep>,
    pub homework_assignment: String,
}

pub struct NcertLessonPlanGenerator;

impl NcertLessonPlanGenerator {
    pub fn generate_5e_lesson_plan(
        grade: NcertClassGrade,
        chapter_title: &str,
        duration_mins: u16,
    ) -> NcertLessonPlan {
        let per_phase_dur = duration_mins / 5;

        NcertLessonPlan {
            topic: chapter_title.to_string(),
            grade,
            learning_objectives: vec![
                format!("Understand core NCERT concepts of {}", chapter_title),
                format!("Apply algebraic/geometric properties to solve Class {} exercises", grade as u8),
            ],
            steps: vec![
                NcertLessonPlanStep {
                    phase: Phase5E::Engage,
                    duration_minutes: per_phase_dur,
                    teacher_activity: format!("Present real-world problem introducing {}", chapter_title),
                    student_activity: "Brainstorm and respond to teacher prompts".to_string(),
                    blackboard_teaching_aids: "Smart Board visual diagram".to_string(),
                },
                NcertLessonPlanStep {
                    phase: Phase5E::Explore,
                    duration_minutes: per_phase_dur,
                    teacher_activity: "Guide students through textbook examples".to_string(),
                    student_activity: "Work in pairs on NCERT Activity/Try These".to_string(),
                    blackboard_teaching_aids: "NCERT Exercise figure".to_string(),
                },
                NcertLessonPlanStep {
                    phase: Phase5E::Explain,
                    duration_minutes: per_phase_dur,
                    teacher_activity: "Derive key NCERT formulas on the board step-by-step".to_string(),
                    student_activity: "Take notes and ask clarifying questions".to_string(),
                    blackboard_teaching_aids: "Formula derivation LaTeX proof".to_string(),
                },
                NcertLessonPlanStep {
                    phase: Phase5E::Elaborate,
                    duration_minutes: per_phase_dur,
                    teacher_activity: "Assign higher-order thinking (HOTs) NCERT Exemplar problem".to_string(),
                    student_activity: "Solve complex application questions".to_string(),
                    blackboard_teaching_aids: "Step-by-step hint outline".to_string(),
                },
                NcertLessonPlanStep {
                    phase: Phase5E::Evaluate,
                    duration_minutes: per_phase_dur,
                    teacher_activity: "Conduct 5-minute exit ticket quiz".to_string(),
                    student_activity: "Submit quick individual solution".to_string(),
                    blackboard_teaching_aids: "Quiz answer key".to_string(),
                },
            ],
            homework_assignment: format!("Solve NCERT Exercise questions from {}", chapter_title),
        }
    }
}

// ============================================================================
// 📐 4. QUESTION BANK & CBSE TEST PAPER GENERATOR
// ============================================================================

/// Question Types matching CBSE / NCERT assessment patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NcertQuestionType {
    Mcq1Mark,
    AssertionReason1Mark,
    ShortAnswer2Marks,
    ShortAnswer3Marks,
    LongAnswer5Marks,
    CaseBased4Marks,
}

/// Bloom's Taxonomy Cognitive Level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BloomsTaxonomyLevel {
    Remembering,
    Understanding,
    Applying,
    Analyzing,
    Evaluating,
    Creating,
}

#[derive(Debug, Clone)]
pub struct NcertQuestion {
    pub question_id: u32,
    pub question_text: String,
    pub q_type: NcertQuestionType,
    pub bloom_level: BloomsTaxonomyLevel,
    pub marks: u8,
    pub marking_scheme_steps: Vec<String>,
}

pub struct NcertQuestionBankManager {
    pub bank: Vec<NcertQuestion>,
}

impl NcertQuestionBankManager {
    pub fn new() -> Self {
        let mut mgr = Self { bank: Vec::new() };
        mgr.seed_sample_questions();
        mgr
    }

    fn seed_sample_questions(&mut self) {
        self.bank.push(NcertQuestion {
            question_id: 101,
            question_text: "Find the zeroes of the polynomial x^2 - 3x - 10.".to_string(),
            q_type: NcertQuestionType::ShortAnswer2Marks,
            bloom_level: BloomsTaxonomyLevel::Applying,
            marks: 2,
            marking_scheme_steps: vec![
                "Factorize: (x - 5)(x + 2) = 0 [1 Mark]".to_string(),
                "Zeroes are x = 5 and x = -2 [1 Mark]".to_string(),
            ],
        });

        self.bank.push(NcertQuestion {
            question_id: 102,
            question_text: "Evaluate integral of (x^2 + 1) dx.".to_string(),
            q_type: NcertQuestionType::ShortAnswer2Marks,
            bloom_level: BloomsTaxonomyLevel::Applying,
            marks: 2,
            marking_scheme_steps: vec![
                "Split integral: \\int x^2 dx + \\int 1 dx [1 Mark]".to_string(),
                "Result: x^3 / 3 + x + C [1 Mark]".to_string(),
            ],
        });
    }

    pub fn add_question(&mut self, q: NcertQuestion) {
        self.bank.push(q);
    }

    pub fn generate_cbse_question_paper(&self, target_total_marks: u8) -> Vec<NcertQuestion> {
        let mut selected = Vec::new();
        let mut current_marks = 0u8;

        for q in &self.bank {
            if current_marks + q.marks <= target_total_marks {
                selected.push(q.clone());
                current_marks += q.marks;
            }
        }
        selected
    }
}

impl Default for NcertQuestionBankManager {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 📐 5. STEP-BY-STEP NCERT EXERCISE SOLUTION SOLVER
// ============================================================================

#[derive(Debug, Clone)]
pub struct StepByStepSolution {
    pub problem_statement: String,
    pub given_data: Vec<String>,
    pub formulas_used: Vec<String>,
    pub step_by_step_derivation: Vec<String>,
    pub final_answer: String,
}

pub struct NcertStepByStepSolutionSolver;

impl NcertStepByStepSolutionSolver {
    pub fn solve_quadratic_equation(a: f64, b: f64, c: f64) -> StepByStepSolution {
        let disc = b * b - 4.0 * a * c;
        let mut derivation = Vec::new();
        derivation.push(format!("Identify coefficients: a = {}, b = {}, c = {}", a, b, c));
        derivation.push(format!("Calculate Discriminant D = b^2 - 4ac = ({})^2 - 4({})({}) = {}", b, a, c, disc));

        let answer = if disc >= 0.0 {
            let r1 = (-b + disc.sqrt()) / (2.0 * a);
            let r2 = (-b - disc.sqrt()) / (2.0 * a);
            derivation.push(format!("D >= 0, Real roots exist: x = (-b +- sqrt(D)) / 2a"));
            derivation.push(format!("x1 = (-({}) + {}) / {} = {}", b, disc.sqrt(), 2.0 * a, r1));
            derivation.push(format!("x2 = (-({}) - {}) / {} = {}", b, disc.sqrt(), 2.0 * a, r2));
            format!("Roots: x = {}, {}", r1, r2)
        } else {
            derivation.push("D < 0, No real roots exist (Complex conjugate roots)".to_string());
            "No real roots".to_string()
        };

        StepByStepSolution {
            problem_statement: format!("Solve {}x^2 + {}x + {} = 0", a, b, c),
            given_data: vec![format!("a = {}", a), format!("b = {}", b), format!("c = {}", c)],
            formulas_used: vec!["D = b^2 - 4ac".to_string(), "x = (-b +- \\sqrt{D}) / (2a)".to_string()],
            step_by_step_derivation: derivation,
            final_answer: answer,
        }
    }

    pub fn solve_2x2_determinant(a11: f64, a12: f64, a21: f64, a22: f64) -> StepByStepSolution {
        let det = a11 * a22 - a12 * a21;
        StepByStepSolution {
            problem_statement: format!("Calculate |[{}, {}; {}, {}]|", a11, a12, a21, a22),
            given_data: vec![format!("Matrix A = [[{}, {}], [{}, {}]]", a11, a12, a21, a22)],
            formulas_used: vec!["det(A) = a11*a22 - a12*a21".to_string()],
            step_by_step_derivation: vec![
                format!("Multiply primary diagonal: {} * {} = {}", a11, a22, a11 * a22),
                format!("Multiply secondary diagonal: {} * {} = {}", a12, a21, a12 * a21),
                format!("Subtract: {} - {} = {}", a11 * a22, a12 * a21, det),
            ],
            final_answer: format!("Determinant = {}", det),
        }
    }
}

// ============================================================================
// 📐 6. TEACHER ANALYTICS & INTERNAL ASSESSMENT EVALUATOR
// ============================================================================

#[derive(Debug, Clone)]
pub struct StudentAssessmentEntry {
    pub student_roll_no: u32,
    pub student_name: String,
    pub periodic_test_marks: f32, // Out of 10
    pub notebook_submission_marks: f32, // Out of 5
    pub maths_lab_activity_marks: f32, // Out of 5
}

pub struct NcertTeacherAnalyticsEngine {
    pub student_records: Vec<StudentAssessmentEntry>,
}

impl NcertTeacherAnalyticsEngine {
    pub fn new() -> Self {
        Self {
            student_records: Vec::new(),
        }
    }

    pub fn record_student_assessment(&mut self, entry: StudentAssessmentEntry) {
        self.student_records.push(entry);
    }

    pub fn calculate_internal_assessment_20_marks(&self, roll_no: u32) -> Option<f32> {
        self.student_records.iter().find(|s| s.student_roll_no == roll_no).map(|s| {
            s.periodic_test_marks + s.notebook_submission_marks + s.maths_lab_activity_marks
        })
    }

    pub fn compute_class_average_internal_marks(&self) -> f32 {
        if self.student_records.is_empty() {
            return 0.0;
        }
        let total: f32 = self
            .student_records
            .iter()
            .map(|s| s.periodic_test_marks + s.notebook_submission_marks + s.maths_lab_activity_marks)
            .sum();
        total / self.student_records.len() as f32
    }
}

impl Default for NcertTeacherAnalyticsEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_ncert_formula_repository_latex() {
        let repo = NcertMathsFormulaRepository::new();
        let latex = repo.render_blackboard_latex(10, 8);
        assert!(latex.contains("Introduction to Trigonometry"));
        assert!(latex.contains("\\sin^2 \\theta + \\cos^2 \\theta = 1"));
    }

    #[test]
    fn test_lesson_plan_generator_5e() {
        let plan = NcertLessonPlanGenerator::generate_5e_lesson_plan(
            NcertClassGrade::Class10,
            "Polynomials",
            50,
        );
        assert_eq!(plan.steps.len(), 5);
        assert_eq!(plan.steps[0].phase, Phase5E::Engage);
        assert_eq!(plan.steps[0].duration_minutes, 10);
    }

    #[test]
    fn test_question_bank_manager() {
        let q_bank = NcertQuestionBankManager::new();
        let test_paper = q_bank.generate_cbse_question_paper(4);
        assert_eq!(test_paper.len(), 2); // 2 + 2 = 4 marks
    }

    #[test]
    fn test_step_by_step_solution_solver() {
        let quad_sol = NcertStepByStepSolutionSolver::solve_quadratic_equation(1.0, -3.0, -10.0);
        assert_eq!(quad_sol.final_answer, "Roots: x = 5, -2");

        let det_sol = NcertStepByStepSolutionSolver::solve_2x2_determinant(4.0, 2.0, 1.0, 3.0);
        assert_eq!(det_sol.final_answer, "Determinant = 10");
    }

    #[test]
    fn test_teacher_analytics_engine() {
        let mut analytics = NcertTeacherAnalyticsEngine::new();
        analytics.record_student_assessment(StudentAssessmentEntry {
            student_roll_no: 1,
            student_name: "Aarav Sharma".to_string(),
            periodic_test_marks: 8.5,
            notebook_submission_marks: 4.5,
            maths_lab_activity_marks: 5.0,
        });

        let internal_marks = analytics.calculate_internal_assessment_20_marks(1).unwrap();
        assert_eq!(internal_marks, 18.0);
        assert_eq!(analytics.compute_class_average_internal_marks(), 18.0);
    }
}
