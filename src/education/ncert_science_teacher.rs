use std::boxed::Box;
use std::format;
use std::string::{String, ToString};
use std::vec;
use std::vec::Vec;

use crate::klib::collections::HashMap;

/// Grade/Class level according to NCERT Science Curriculum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NcertGrade {
    Class6,
    Class7,
    Class8,
    Class9,
    Class10,
    Class11Physics,
    Class11Chemistry,
    Class11Biology,
    Class12Physics,
    Class12Chemistry,
    Class12Biology,
}

/// Subject Domain in NCERT Science
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NcertSubjectDomain {
    Physics,
    Chemistry,
    Biology,
    EnvironmentalScience,
}

/// Bloom's Taxonomy Cognitive Level for Questions & Objectives
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BloomsTaxonomyLevel {
    Remembering,
    Understanding,
    Applying,
    Analyzing,
    Evaluating,
    Creating,
}

/// NCERT Science Chapter Topic Module
#[derive(Debug, Clone)]
pub struct NcertChapterTopic {
    pub chapter_number: u32,
    pub title: String,
    pub domain: NcertSubjectDomain,
    pub grade: NcertGrade,
    pub key_concepts: Vec<String>,
    pub practical_experiments: Vec<String>,
    pub learning_objectives: Vec<String>,
}

/// Lesson Plan Generator Model for Teachers
#[derive(Debug, Clone)]
pub struct NcertLessonPlan {
    pub topic_title: String,
    pub grade: NcertGrade,
    pub duration_minutes: u32,
    pub learning_objectives: Vec<String>,
    pub required_lab_apparatus: Vec<String>,
    pub teaching_methodology: String,
    pub step_by_step_pedagogy: Vec<String>,
    pub assessment_rubric: Vec<String>,
    pub safety_precautions: Vec<String>,
}

/// Question Bank Entry following CBSE NCERT Examination Standards
#[derive(Debug, Clone)]
pub struct NcertQuestionItem {
    pub id: u32,
    pub grade: NcertGrade,
    pub chapter: String,
    pub question_text: String,
    pub cognitive_level: BloomsTaxonomyLevel,
    pub marks: u32,
    pub answer_key: String,
    pub marking_scheme_steps: Vec<String>,
}

/// Virtual Science Lab Experiment Simulator for Demonstrations
#[derive(Debug, Clone)]
pub struct NcertVirtualLabExperiment {
    pub experiment_id: String,
    pub title: String,
    pub domain: NcertSubjectDomain,
    pub safety_hazard_level: String, // "Low", "Medium", "High (Fume Hood Required)"
    pub apparatus_checklist: Vec<String>,
    pub chemical_reagents: Vec<String>,
    pub simulated_output: String,
}

/// Professional Tool Suite for NCERT Science Educators
pub struct NcertScienceTeacherSuite {
    pub curriculum_database: Vec<NcertChapterTopic>,
    pub question_bank: Vec<NcertQuestionItem>,
    pub virtual_labs: Vec<NcertVirtualLabExperiment>,
}

impl NcertScienceTeacherSuite {
    pub fn new() -> Self {
        let mut suite = Self {
            curriculum_database: Vec::new(),
            question_bank: Vec::new(),
            virtual_labs: Vec::new(),
        };
        suite.load_default_ncert_curriculum();
        suite.load_default_question_bank();
        suite.load_default_virtual_labs();
        suite
    }

    /// Loads built-in NCERT Science curriculum topics (Classes 6-12)
    fn load_default_ncert_curriculum(&mut self) {
        self.curriculum_database.push(NcertChapterTopic {
            chapter_number: 1,
            title: String::from("Chemical Reactions and Equations"),
            domain: NcertSubjectDomain::Chemistry,
            grade: NcertGrade::Class10,
            key_concepts: vec![
                String::from("Balanced Chemical Equations"),
                String::from("Combination, Decomposition, Displacement Reactions"),
                String::from("Redox & Oxidation-Reduction"),
                String::from("Corrosion and Rancidity"),
            ],
            practical_experiments: vec![
                String::from("Burning of Magnesium ribbon in air"),
                String::from("Reaction of Quicklime with Water"),
                String::from("Displacement reaction of Iron nail in Copper Sulphate solution"),
            ],
            learning_objectives: vec![
                String::from("Formulate balanced chemical equations"),
                String::from("Identify reaction types from observations"),
            ],
        });

        self.curriculum_database.push(NcertChapterTopic {
            chapter_number: 10,
            title: String::from("Light - Reflection and Refraction"),
            domain: NcertSubjectDomain::Physics,
            grade: NcertGrade::Class10,
            key_concepts: vec![
                String::from("Laws of Reflection"),
                String::from("Spherical Mirrors & Ray Diagrams"),
                String::from("Mirror Formula & Lens Formula"),
                String::from("Refractive Index & Snell's Law"),
            ],
            practical_experiments: vec![
                String::from("Tracing the path of ray of light passing through glass slab"),
                String::from("Finding focal length of concave mirror and convex lens"),
            ],
            learning_objectives: vec![
                String::from("Construct ray diagrams for spherical lenses and mirrors"),
                String::from("Calculate image distance and magnification using lens formula"),
            ],
        });

        self.curriculum_database.push(NcertChapterTopic {
            chapter_number: 6,
            title: String::from("Life Processes"),
            domain: NcertSubjectDomain::Biology,
            grade: NcertGrade::Class10,
            key_concepts: vec![
                String::from("Autotrophic and Heterotrophic Nutrition"),
                String::from("Respiration in Humans & Plants"),
                String::from("Transportation in Human Beings (Heart/Blood)"),
                String::from("Excretion and Nephron structure"),
            ],
            practical_experiments: vec![
                String::from("Demonstrating light is necessary for photosynthesis"),
                String::from("Observing temporary mount of leaf peel for stomata"),
            ],
            learning_objectives: vec![
                String::from("Illustrating double circulation in human heart"),
                String::from("Analyzing aerobic vs anaerobic breakdown of glucose"),
            ],
        });

        self.curriculum_database.push(NcertChapterTopic {
            chapter_number: 2,
            title: String::from("Electrostatic Potential and Capacitance"),
            domain: NcertSubjectDomain::Physics,
            grade: NcertGrade::Class12Physics,
            key_concepts: vec![
                String::from("Electric Potential due to a Dipole"),
                String::from("Equipotential Surfaces"),
                String::from("Capacitors and Capacitance"),
                String::from("Dielectrics and Polarization"),
            ],
            practical_experiments: vec![
                String::from("Charging and discharging of a capacitor"),
            ],
            learning_objectives: vec![
                String::from("Derive expression for energy stored in a parallel plate capacitor"),
            ],
        });
    }

    /// Loads built-in CBSE NCERT pattern question bank
    fn load_default_question_bank(&mut self) {
        self.question_bank.push(NcertQuestionItem {
            id: 101,
            grade: NcertGrade::Class10,
            chapter: String::from("Chemical Reactions and Equations"),
            question_text: String::from("Why does the color of copper sulphate solution change when an iron nail is dipped in it? Write the balanced chemical equation."),
            cognitive_level: BloomsTaxonomyLevel::Understanding,
            marks: 3,
            answer_key: String::from("Iron is more reactive than copper. It displaces copper from copper sulphate solution forming iron sulphate (greenish) and copper metal precipitation."),
            marking_scheme_steps: vec![
                String::from("1 Mark: Statement explaining displacement due to reactivity series"),
                String::from("1 Mark: Observation of color change from blue to light green"),
                String::from("1 Mark: Fe(s) + CuSO4(aq) -> FeSO4(aq) + Cu(s)"),
            ],
        });

        self.question_bank.push(NcertQuestionItem {
            id: 102,
            grade: NcertGrade::Class10,
            chapter: String::from("Light - Reflection and Refraction"),
            question_text: String::from("An object 5 cm in length is held 25 cm away from a converging lens of focal length 10 cm. Find the position, size and nature of the image formed."),
            cognitive_level: BloomsTaxonomyLevel::Applying,
            marks: 5,
            answer_key: String::from("u = -25 cm, f = +10 cm. Using 1/v - 1/u = 1/f -> v = +16.67 cm. Magnification m = v/u = -0.667. Image is real, inverted, and diminished."),
            marking_scheme_steps: vec![
                String::from("1 Mark: Correct sign convention for u and f"),
                String::from("2 Marks: Correct application of Lens Formula and v calculation"),
                String::from("1 Mark: Magnification and image size calculation"),
                String::from("1 Mark: Stating nature (Real and Inverted)"),
            ],
        });
    }

    /// Loads interactive Virtual Science Lab demonstrations
    fn load_default_virtual_labs(&mut self) {
        self.virtual_labs.push(NcertVirtualLabExperiment {
            experiment_id: String::from("VLAB-CHEM-01"),
            title: String::from("Decomposition of Lead Nitrate"),
            domain: NcertSubjectDomain::Chemistry,
            safety_hazard_level: String::from("High (Fume Hood Required - NO2 Gas toxic)"),
            apparatus_checklist: vec![
                String::from("Boiling tube"),
                String::from("Test tube holder"),
                String::from("Bunsen burner"),
            ],
            chemical_reagents: vec![String::from("Lead nitrate powder Pb(NO3)2")],
            simulated_output: String::from("Emission of brown fumes of Nitrogen Dioxide (NO2) and yellow residue of Lead Oxide (PbO) remaining in tube."),
        });

        self.virtual_labs.push(NcertVirtualLabExperiment {
            experiment_id: String::from("VLAB-PHYS-01"),
            title: String::from("Ohm's Law Verification"),
            domain: NcertSubjectDomain::Physics,
            safety_hazard_level: String::from("Low"),
            apparatus_checklist: vec![
                String::from("Ammeter"),
                String::from("Voltmeter"),
                String::from("Rheostat"),
                String::from("Resistor wire"),
                String::from("DC Battery Source"),
            ],
            chemical_reagents: vec![],
            simulated_output: String::from("V-I graph yields a straight line passing through origin, verifying V = I * R constant resistance ratio."),
        });
    }

    /// Generates a standardized NCERT Pedagogy 5E Lesson Plan for Teachers
    pub fn generate_lesson_plan(&self, chapter_title: &str, grade: NcertGrade) -> Result<NcertLessonPlan, &'static str> {
        let topic = self
            .curriculum_database
            .iter()
            .find(|t| t.title.to_lowercase() == chapter_title.to_lowercase() && t.grade == grade)
            .ok_or("Topic not found in NCERT curriculum database")?;

        Ok(NcertLessonPlan {
            topic_title: topic.title.clone(),
            grade: topic.grade,
            duration_minutes: 45,
            learning_objectives: topic.learning_objectives.clone(),
            required_lab_apparatus: topic.practical_experiments.clone(),
            teaching_methodology: String::from("NCERT 5E Model (Engage, Explore, Explain, Elaborate, Evaluate)"),
            step_by_step_pedagogy: vec![
                String::from("Engage (5m): Show real-world phenomenon video or demonstration"),
                String::from("Explore (10m): Hands-on group activity or Virtual Lab simulation"),
                String::from("Explain (15m): Concept derivation and NCERT textbook diagram analysis"),
                String::from("Elaborate (10m): Numerical problem solving & daily life applications"),
                String::from("Evaluate (5m): Formative exit ticket questions"),
            ],
            assessment_rubric: vec![
                String::from("Understanding of core definitions (20%)"),
                String::from("Diagram / Equation accuracy (30%)"),
                String::from("Application & Problem solving (30%)"),
                String::from("Lab practical participation (20%)"),
            ],
            safety_precautions: vec![
                String::from("Wear safety goggles during chemical heating"),
                String::from("Ensure tight electrical connections to avoid sparking"),
            ],
        })
    }

    /// Generates a custom CBSE pattern Question Paper with answer key & marking scheme
    pub fn generate_exam_paper(&self, grade: NcertGrade, total_marks: u32) -> (String, Vec<NcertQuestionItem>) {
        let matching_questions: Vec<NcertQuestionItem> = self
            .question_bank
            .iter()
            .filter(|q| q.grade == grade)
            .cloned()
            .collect();

        let mut selected = Vec::new();
        let mut current_marks = 0;

        for q in matching_questions {
            if current_marks + q.marks <= total_marks {
                current_marks += q.marks;
                selected.push(q);
            }
        }

        let paper_header = format!(
            "CENTRAL BOARD OF SECONDARY EDUCATION - NCERT SCIENCE EXAMINATION\nGrade: {:?} | Maximum Marks: {} | Duration: 3 Hours\nInstruction: Read all questions carefully. Draw neat diagrams wherever required.\n",
            grade, total_marks
        );

        (paper_header, selected)
    }

    /// Simulates Virtual Lab Experiment for Classroom Interactive Demonstration
    pub fn run_virtual_lab_demo(&self, exp_id: &str) -> Result<String, &'static str> {
        let lab = self
            .virtual_labs
            .iter()
            .find(|l| l.experiment_id == exp_id)
            .ok_or("Virtual Lab experiment not found")?;

        Ok(format!(
            "--- NCERT VIRTUAL LAB SIMULATION: {} ---\nHazard Warning: {}\nApparatus: {}\nSimulated Output Result: {}",
            lab.title,
            lab.safety_hazard_level,
            lab.apparatus_checklist.join(", "),
            lab.simulated_output
        ))
    }
}

impl Default for NcertScienceTeacherSuite {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ncert_lesson_plan_generation() {
        let suite = NcertScienceTeacherSuite::new();
        let plan = suite
            .generate_lesson_plan("Chemical Reactions and Equations", NcertGrade::Class10)
            .unwrap();

        assert_eq!(plan.topic_title, "Chemical Reactions and Equations");
        assert_eq!(plan.duration_minutes, 45);
        assert!(plan.step_by_step_pedagogy.len() >= 5);
    }

    #[test]
    fn test_ncert_exam_paper_generation() {
        let suite = NcertScienceTeacherSuite::new();
        let (header, questions) = suite.generate_exam_paper(NcertGrade::Class10, 10);

        assert!(header.contains("NCERT SCIENCE EXAMINATION"));
        assert!(!questions.is_empty());
        assert_eq!(questions[0].marks, 3);
        assert!(!questions[0].marking_scheme_steps.is_empty());
    }

    #[test]
    fn test_virtual_lab_simulation() {
        let suite = NcertScienceTeacherSuite::new();
        let demo = suite.run_virtual_lab_demo("VLAB-CHEM-01").unwrap();

        assert!(demo.contains("Decomposition of Lead Nitrate"));
        assert!(demo.contains("Nitrogen Dioxide"));
    }
}
