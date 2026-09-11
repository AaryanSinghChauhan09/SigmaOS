// SPDX-License-Identifier: MIT
// Sovereign DPLL SAT-Solver Package Dependency Engine for SigmaOS
// Implements Davis-Putnam-Logemann-Loveland (DPLL) Boolean Satisfiability (SAT) solver
// for zero-ambiguity multi-format cross-distro package dependency resolution.

#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::string::{String, ToString};
#[cfg(not(any(feature = "standalone_test", test)))]
use alloc::vec::Vec;

#[cfg(any(feature = "standalone_test", test))]
use std::string::{String, ToString};
#[cfg(any(feature = "standalone_test", test))]
use std::vec::Vec;

/// Represents a literal variable in SAT CNF (boolean form)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DpllLiteral {
    pub variable_id: usize,
    pub package_name: String,
    pub is_positive: bool,
}

impl DpllLiteral {
    pub fn positive(variable_id: usize, package_name: &str) -> Self {
        Self {
            variable_id,
            package_name: package_name.to_string(),
            is_positive: true,
        }
    }

    pub fn negative(variable_id: usize, package_name: &str) -> Self {
        Self {
            variable_id,
            package_name: package_name.to_string(),
            is_positive: false,
        }
    }

    pub fn negate(&self) -> Self {
        Self {
            variable_id: self.variable_id,
            package_name: self.package_name.clone(),
            is_positive: !self.is_positive,
        }
    }
}

/// Disjunctive clause composed of multiple literals (A OR B OR NOT C)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DpllClause {
    pub literals: Vec<DpllLiteral>,
}

impl DpllClause {
    pub fn new(literals: Vec<DpllLiteral>) -> Self {
        Self { literals }
    }

    pub fn unit_literal(&self) -> Option<DpllLiteral> {
        if self.literals.len() == 1 {
            Some(self.literals[0].clone())
        } else {
            None
        }
    }

    pub fn is_empty(&self) -> bool {
        self.literals.is_empty()
    }
}

/// Variable Assignment state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VariableAssignment {
    Unassigned,
    True,
    False,
}

/// Davis-Putnam-Logemann-Loveland (DPLL) Boolean Satisfiability Solver
pub struct DpllSatSolver {
    pub clauses: Vec<DpllClause>,
    pub variable_count: usize,
}

impl DpllSatSolver {
    pub fn new(clauses: Vec<DpllClause>, variable_count: usize) -> Self {
        Self {
            clauses,
            variable_count,
        }
    }

    /// Solves the CNF formula and returns assigned variables if satisfiable
    pub fn solve(&self) -> Option<Vec<bool>> {
        let mut assignment = vec![VariableAssignment::Unassigned; self.variable_count];
        if self.dpll_recursive(&self.clauses, &mut assignment) {
            Some(
                assignment
                    .iter()
                    .map(|&val| val == VariableAssignment::True)
                    .collect(),
            )
        } else {
            None
        }
    }

    fn dpll_recursive(
        &self,
        clauses: &[DpllClause],
        assignment: &mut Vec<VariableAssignment>,
    ) -> bool {
        // Step 1: Simplify clauses under current assignment
        let mut current_clauses = match self.simplify_clauses(clauses, assignment) {
            Ok(c) => c,
            Err(_) => return false, // Empty clause encountered (conflict)
        };

        if current_clauses.is_empty() {
            return true; // All clauses satisfied
        }

        // Step 2: Unit Propagation
        let mut changed = true;
        while changed {
            changed = false;
            let unit_lit = current_clauses
                .iter()
                .find_map(|clause| clause.unit_literal());

            if let Some(lit) = unit_lit {
                let var_idx = lit.variable_id;
                let desired_val = if lit.is_positive {
                    VariableAssignment::True
                } else {
                    VariableAssignment::False
                };

                if assignment[var_idx] != VariableAssignment::Unassigned
                    && assignment[var_idx] != desired_val
                {
                    return false; // Conflict during unit propagation
                }

                assignment[var_idx] = desired_val;
                current_clauses = match self.simplify_clauses(&current_clauses, assignment) {
                    Ok(c) => c,
                    Err(_) => return false,
                };

                if current_clauses.is_empty() {
                    return true;
                }
                changed = true;
            }
        }

        // Step 3: Pure Literal Elimination
        let pure_lit = self.find_pure_literal(&current_clauses, assignment);
        if let Some(lit) = pure_lit {
            let var_idx = lit.variable_id;
            assignment[var_idx] = if lit.is_positive {
                VariableAssignment::True
            } else {
                VariableAssignment::False
            };
            current_clauses = match self.simplify_clauses(&current_clauses, assignment) {
                Ok(c) => c,
                Err(_) => return false,
            };
            if current_clauses.is_empty() {
                return true;
            }
        }

        // Step 4: Choose unassigned variable and branch (backtracking search)
        let unassigned_var = assignment
            .iter()
            .position(|&val| val == VariableAssignment::Unassigned);

        if let Some(var_idx) = unassigned_var {
            // Try assigning True first
            let mut branch_assignment = assignment.clone();
            branch_assignment[var_idx] = VariableAssignment::True;
            if self.dpll_recursive(&current_clauses, &mut branch_assignment) {
                *assignment = branch_assignment;
                return true;
            }

            // Fallback: Try assigning False
            let mut branch_assignment_false = assignment.clone();
            branch_assignment_false[var_idx] = VariableAssignment::False;
            if self.dpll_recursive(&current_clauses, &mut branch_assignment_false) {
                *assignment = branch_assignment_false;
                return true;
            }
        }

        false
    }

    fn simplify_clauses(
        &self,
        clauses: &[DpllClause],
        assignment: &[VariableAssignment],
    ) -> Result<Vec<DpllClause>, ()> {
        let mut simplified = Vec::new();

        for clause in clauses {
            let mut clause_satisfied = false;
            let mut remaining_literals = Vec::new();

            for lit in &clause.literals {
                let var_val = assignment[lit.variable_id];
                match var_val {
                    VariableAssignment::True => {
                        if lit.is_positive {
                            clause_satisfied = true;
                            break;
                        }
                    }
                    VariableAssignment::False => {
                        if !lit.is_positive {
                            clause_satisfied = true;
                            break;
                        }
                    }
                    VariableAssignment::Unassigned => {
                        remaining_literals.push(lit.clone());
                    }
                }
            }

            if clause_satisfied {
                continue; // Clause removed from formula
            }

            if remaining_literals.is_empty() {
                return Err(()); // Conflict: Clause cannot be satisfied
            }

            simplified.push(DpllClause::new(remaining_literals));
        }

        Ok(simplified)
    }

    fn find_pure_literal(
        &self,
        clauses: &[DpllClause],
        assignment: &[VariableAssignment],
    ) -> Option<DpllLiteral> {
        for var_idx in 0..self.variable_count {
            if assignment[var_idx] != VariableAssignment::Unassigned {
                continue;
            }

            let mut has_pos = false;
            let mut has_neg = false;
            let mut sample_lit = None;

            for clause in clauses {
                for lit in &clause.literals {
                    if lit.variable_id == var_idx {
                        if lit.is_positive {
                            has_pos = true;
                        } else {
                            has_neg = true;
                        }
                        if sample_lit.is_none() {
                            sample_lit = Some(lit.clone());
                        }
                    }
                }
            }

            if (has_pos && !has_neg) || (!has_pos && has_neg) {
                return sample_lit;
            }
        }
        None
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(any(feature = "standalone_test", test))]
mod tests {
    use super::*;

    #[test]
    fn test_dpll_simple_satisfiable() {
        // Clause 1: x0 OR x1
        // Clause 2: NOT x0
        let clauses = vec![
            DpllClause::new(vec![
                DpllLiteral::positive(0, "pkg_a"),
                DpllLiteral::positive(1, "pkg_b"),
            ]),
            DpllClause::new(vec![DpllLiteral::negative(0, "pkg_a")]),
        ];

        let solver = DpllSatSolver::new(clauses, 2);
        let result = solver.solve();

        assert!(result.is_some());
        let assignment = result.unwrap();
        assert!(!assignment[0]); // pkg_a = false
        assert!(assignment[1]);  // pkg_b = true
    }

    #[test]
    fn test_dpll_unsatisfiable_conflict() {
        // Clause 1: x0
        // Clause 2: NOT x0
        let clauses = vec![
            DpllClause::new(vec![DpllLiteral::positive(0, "pkg_a")]),
            DpllClause::new(vec![DpllLiteral::negative(0, "pkg_a")]),
        ];

        let solver = DpllSatSolver::new(clauses, 1);
        let result = solver.solve();

        assert!(result.is_none());
    }

    #[test]
    fn test_dpll_multi_variable_dependency_resolution() {
        // Package A requires B or C: (A => B OR C) => (NOT A OR B OR C)
        // A is selected: (A)
        // B is incompatible with C: (NOT B OR NOT C)
        let clauses = vec![
            DpllClause::new(vec![
                DpllLiteral::negative(0, "pkg_a"),
                DpllLiteral::positive(1, "pkg_b"),
                DpllLiteral::positive(2, "pkg_c"),
            ]),
            DpllClause::new(vec![DpllLiteral::positive(0, "pkg_a")]),
            DpllClause::new(vec![
                DpllLiteral::negative(1, "pkg_b"),
                DpllLiteral::negative(2, "pkg_c"),
            ]),
        ];

        let solver = DpllSatSolver::new(clauses, 3);
        let result = solver.solve();

        assert!(result.is_some());
        let assignment = result.unwrap();
        assert!(assignment[0]); // pkg_a = true
        // Exactly one of pkg_b or pkg_c is true
        assert!(assignment[1] ^ assignment[2]);
    }
}
