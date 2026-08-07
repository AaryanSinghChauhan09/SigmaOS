// SAT Solver for Dependency Resolution
// DPLL (Davis-Putnam-Logemann-Loveland) algorithm implementation

use crate::sigpkg::{Package, Version, VersionConstraint};
use std::collections::{HashMap, HashSet};

// =========================================================================
// Davis-Putnam-Logemann-Loveland (DPLL) Boolean SAT Solver
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Literal {
    pub var_id: usize,
    pub is_positive: bool,
}

impl Literal {
    pub fn new(var_id: usize, is_positive: bool) -> Self {
        Literal { var_id, is_positive }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Clause {
    pub literals: Vec<Literal>,
}

pub struct DpllSolver {
    pub clauses: Vec<Clause>,
    pub num_variables: usize,
}

impl DpllSolver {
    pub fn new(clauses: Vec<Clause>, num_variables: usize) -> Self {
        DpllSolver { clauses, num_variables }
    }

    /// Evaluates a clause under the current assignment.
    /// Returns Some(true) if satisfied, Some(false) if unsatisfied (conflict), or None if unresolved.
    pub fn evaluate_clause(&self, clause: &Clause, assignment: &HashMap<usize, bool>) -> Option<bool> {
        let mut has_unassigned = false;
        for lit in &clause.literals {
            if let Some(&val) = assignment.get(&lit.var_id) {
                if val == lit.is_positive {
                    return Some(true); // At least one literal is satisfied
                }
            } else {
                has_unassigned = true;
            }
        }
        if has_unassigned {
            None // Clause is unresolved
        } else {
            Some(false) // Clause is unsatisfied (conflict!)
        }
    }

    /// Performs Unit Propagation:
    /// If an unresolved clause has only one unassigned literal, that literal must be assigned to true.
    pub fn unit_propagate(&self, assignment: &mut HashMap<usize, bool>) -> Result<bool, ()> {
        let mut changed = false;
        loop {
            let mut unit_literal = None;
            for clause in &self.clauses {
                // If clause is already satisfied, skip
                if self.evaluate_clause(clause, assignment) == Some(true) {
                    continue;
                }

                // Count unassigned literals
                let mut unassigned = Vec::new();
                let mut is_conflict = true;

                for lit in &clause.literals {
                    if let Some(&val) = assignment.get(&lit.var_id) {
                        if val == lit.is_positive {
                            is_conflict = false;
                        }
                    } else {
                        unassigned.push(*lit);
                    }
                }

                if is_conflict {
                    if unassigned.len() == 1 {
                        unit_literal = Some(unassigned[0]);
                        break;
                    } else if unassigned.is_empty() {
                        return Err(()); // Unsatisfied clause -> Conflict!
                    }
                }
            }

            if let Some(lit) = unit_literal {
                assignment.insert(lit.var_id, lit.is_positive);
                changed = true;
            } else {
                break;
            }
        }
        Ok(changed)
    }

    /// Performs Pure Literal Elimination:
    /// If a variable appears with only one polarity in all unsatisfied clauses, assign it accordingly.
    pub fn pure_literal_elimination(&self, assignment: &mut HashMap<usize, bool>) -> bool {
        let mut changed = false;
        for var_id in 0..self.num_variables {
            if assignment.contains_key(&var_id) {
                continue;
            }

            let mut has_positive = false;
            let mut has_negative = false;

            for clause in &self.clauses {
                if self.evaluate_clause(clause, assignment) == Some(true) {
                    continue;
                }
                for lit in &clause.literals {
                    if lit.var_id == var_id {
                        if lit.is_positive {
                            has_positive = true;
                        } else {
                            has_negative = true;
                        }
                    }
                }
            }

            if has_positive && !has_negative {
                assignment.insert(var_id, true);
                changed = true;
            } else if !has_positive && has_negative {
                assignment.insert(var_id, false);
                changed = true;
            }
        }
        changed
    }

    /// Solves the CNF formula recursively using the backtracking DPLL search.
    pub fn solve(&self) -> Option<HashMap<usize, bool>> {
        let mut assignment = HashMap::new();
        self.solve_recursive(&mut assignment, 0)
    }

    fn solve_recursive(&self, assignment: &mut HashMap<usize, bool>, next_var: usize) -> Option<HashMap<usize, bool>> {
        // Step 1: Unit Propagation & Pure Literal Elimination
        let mut local_assignment = assignment.clone();
        if self.unit_propagate(&mut local_assignment).is_err() {
            return None; // Conflict!
        }
        self.pure_literal_elimination(&mut local_assignment);

        // Step 2: Check if all clauses are satisfied
        let mut all_satisfied = true;
        for clause in &self.clauses {
            if self.evaluate_clause(clause, &local_assignment) != Some(true) {
                all_satisfied = false;
                break;
            }
        }
        if all_satisfied {
            return Some(local_assignment);
        }

        // Step 3: Find next unassigned variable to branch
        let mut branch_var = None;
        for v in next_var..self.num_variables {
            if !local_assignment.contains_key(&v) {
                branch_var = Some(v);
                break;
            }
        }

        let var_id = match branch_var {
            Some(id) => id,
            None => return Some(local_assignment), // Everything assigned and satisfied
        };

        // Step 4: Branch on branch_var = true
        let mut assign_true = local_assignment.clone();
        assign_true.insert(var_id, true);
        if let Some(res) = self.solve_recursive(&mut assign_true, var_id + 1) {
            return Some(res);
        }

        // Step 5: Branch on branch_var = false (Backtrack)
        let mut assign_false = local_assignment;
        assign_false.insert(var_id, false);
        if let Some(res) = self.solve_recursive(&mut assign_false, var_id + 1) {
            return Some(res);
        }

        None
    }
}

// =========================================================================
// SatSolver (Package dependency resolver wrapper)
// =========================================================================

pub struct SatSolver {
    pub packages: HashMap<String, Vec<Package>>,
}

impl SatSolver {
    pub fn new() -> Self {
        SatSolver {
            packages: HashMap::new(),
        }
    }

    pub fn add_package(&mut self, package: Package) {
        self.packages
            .entry(package.name.clone())
            .or_default()
            .push(package);
    }

    /// Resolve dependencies for target package using our advanced DPLL SAT Solver Engine!
    pub fn resolve(
        &self,
        package_name: &str,
        version_constraint: &VersionConstraint,
    ) -> Result<Vec<Package>, ResolveError> {
        if self.detect_circular(package_name) {
            return Err(ResolveError::CircularDependency(package_name.to_string()));
        }

        // 1. Gather all candidates recursively
        let mut all_packages = Vec::new();
        let mut queue = Vec::new();
        let mut queued_names = HashSet::new();

        queue.push((package_name.to_string(), version_constraint.clone()));
        queued_names.insert(package_name.to_string());

        while let Some((p_name, constraint)) = queue.pop() {
            let pkgs = self
                .packages
                .get(&p_name)
                .ok_or(ResolveError::PackageNotFound(p_name.clone()))?;

            let mut matched = false;
            for p in pkgs {
                if self.satisfies_constraint(&p.version, &constraint) {
                    matched = true;
                    all_packages.push(p.clone());

                    // Enqueue dependencies
                    for dep in &p.dependencies {
                        if !queued_names.contains(&dep.name) {
                            queued_names.insert(dep.name.clone());
                            queue.push((dep.name.clone(), dep.version_constraint.clone()));
                        }
                    }
                }
            }

            if !matched {
                return Err(ResolveError::NoMatchingVersion(p_name));
            }
        }

        // 2. Map candidate packages to unique variables IDs
        let mut pkg_to_var = HashMap::new();
        let mut var_to_pkg = HashMap::new();
        for (idx, p) in all_packages.iter().enumerate() {
            let identifier = format!("{}#{}", p.name, p.version);
            pkg_to_var.insert(identifier.clone(), idx);
            var_to_pkg.insert(idx, p.clone());
        }

        let num_variables = all_packages.len();
        let mut clauses = Vec::new();

        // 3. Build CNF clauses:
        // A. We MUST install target package (at least one candidate matching constraint)
        let mut target_lits = Vec::new();
        for p in &all_packages {
            if p.name == package_name && self.satisfies_constraint(&p.version, version_constraint) {
                let id = *pkg_to_var.get(&format!("{}#{}", p.name, p.version)).unwrap();
                target_lits.push(Literal::new(id, true));
            }
        }
        if target_lits.is_empty() {
            return Err(ResolveError::NoMatchingVersion(package_name.to_string()));
        }
        clauses.push(Clause { literals: target_lits });

        // B. If a package is installed, its dependencies must also be satisfied (NOT P or D1_v1 or D1_v2)
        for p in &all_packages {
            let p_id = *pkg_to_var.get(&format!("{}#{}", p.name, p.version)).unwrap();

            for dep in &p.dependencies {
                let mut dep_lits = Vec::new();
                dep_lits.push(Literal::new(p_id, false)); // NOT P

                for cand in &all_packages {
                    if cand.name == dep.name && self.satisfies_constraint(&cand.version, &dep.version_constraint) {
                        let c_id = *pkg_to_var.get(&format!("{}#{}", cand.name, cand.version)).unwrap();
                        dep_lits.push(Literal::new(c_id, true)); // cand_v
                    }
                }

                clauses.push(Clause { literals: dep_lits });
            }
        }

        // C. Duplicate candidate versions of the same package conflict (e.g. NOT p_v1 or NOT p_v2)
        for i in 0..num_variables {
            let p1 = var_to_pkg.get(&i).unwrap();
            for j in (i + 1)..num_variables {
                let p2 = var_to_pkg.get(&j).unwrap();
                if p1.name == p2.name {
                    clauses.push(Clause {
                        literals: vec![Literal::new(i, false), Literal::new(j, false)],
                    });
                }
            }
        }

        // 4. Solve CNF using DPLLSAT solver
        let dpll = DpllSolver::new(clauses, num_variables);
        if let Some(assignment) = dpll.solve() {
            let mut resolved = Vec::new();
            for (var_id, &val) in &assignment {
                if val {
                    resolved.push(var_to_pkg.get(var_id).unwrap().clone());
                }
            }
            Ok(resolved)
        } else {
            Err(ResolveError::Conflict(package_name.to_string()))
        }
    }

    pub fn satisfies_constraint(&self, version: &Version, constraint: &VersionConstraint) -> bool {
        match constraint {
            VersionConstraint::Exact(v) => version == v,
            VersionConstraint::GreaterThan(v) => version > v,
            VersionConstraint::LessThan(v) => version < v,
            VersionConstraint::GreaterOrEqual(v) => version >= v,
            VersionConstraint::LessOrEqual(v) => version <= v,
            VersionConstraint::Any => true,
        }
    }

    pub fn detect_circular(&self, package_name: &str) -> bool {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        self.has_cycle(package_name, &mut visited, &mut recursion_stack)
    }

    fn has_cycle(
        &self,
        package_name: &str,
        visited: &mut HashSet<String>,
        recursion_stack: &mut HashSet<String>,
    ) -> bool {
        visited.insert(package_name.to_string());
        recursion_stack.insert(package_name.to_string());

        if let Some(packages) = self.packages.get(package_name) {
            for package in packages {
                for dep in &package.dependencies {
                    if !visited.contains(&dep.name) {
                        if self.has_cycle(&dep.name, visited, recursion_stack) {
                            return true;
                        }
                    } else if recursion_stack.contains(&dep.name) {
                        return true;
                    }
                }
            }
        }

        recursion_stack.remove(package_name);
        false
    }
}

impl Default for SatSolver {
    fn default() -> Self {
        SatSolver::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolveError {
    PackageNotFound(String),
    NoMatchingVersion(String),
    CircularDependency(String),
    Conflict(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sigpkg::Dependency;

    #[test]
    fn test_sat_solver_creation() {
        let solver = SatSolver::new();
        assert!(solver.packages.is_empty());
    }

    #[test]
    fn test_add_package() {
        let mut solver = SatSolver::new();
        let package = Package {
            name: "test".to_string(),
            version: Version::new(1, 0, 0),
            description: String::new(),
            dependencies: Vec::new(),
            checksum: String::new(),
        };
        solver.add_package(package);
        assert!(solver.packages.contains_key("test"));
    }

    #[test]
    fn test_version_constraint_satisfaction() {
        let solver = SatSolver::new();
        let v1 = Version::new(1, 0, 0);
        let v2 = Version::new(1, 0, 1);

        assert!(solver.satisfies_constraint(&v2, &VersionConstraint::GreaterThan(v1)));
        assert!(solver.satisfies_constraint(&v1, &VersionConstraint::LessThan(v2)));
        assert!(solver.satisfies_constraint(&v1, &VersionConstraint::Exact(v1)));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut solver = SatSolver::new();

        let pkg_a = Package {
            name: "A".to_string(),
            version: Version::new(1, 0, 0),
            description: String::new(),
            dependencies: vec![Dependency {
                name: "B".to_string(),
                version_constraint: VersionConstraint::Any,
            }],
            String::new(),
        );

        let pkg_b = Package::new(
            "B".to_string(),
            Version::new(1, 0, 0),
            String::new(),
            vec![Dependency {
                name: "A".to_string(),
                version_constraint: VersionConstraint::Any,
            }],
            String::new(),
        );

        solver.add_package(pkg_a);
        solver.add_package(pkg_b);

        assert!(solver.detect_circular("A"));
    }

    #[test]
    fn test_dpll_solving() {
        // Setup simple CNF: (v0 or v1) and (-v0 or -v1)
        let clauses = vec![
            Clause { literals: vec![Literal::new(0, true), Literal::new(1, true)] },
            Clause { literals: vec![Literal::new(0, false), Literal::new(1, false)] },
        ];
        let dpll = DpllSolver::new(clauses, 2);
        let assignment = dpll.solve().unwrap();
        assert!(assignment.get(&0).unwrap() != assignment.get(&1).unwrap());
    }
}
