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
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// Interim OS and MNT Reform compatibility subsystem for SigmaOS
// Implements a safe, zero-dependency Lisp interpreter VM and MNT LPC telemetry drivers.

extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

// ==========================================
// 1. INTERIM LISP INTERPRETER VM
// ==========================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LispVal {
    Atom(String),
    Number(i32),
    List(Vec<LispVal>),
}

/// S-Expression Lisp VM mimicking Interim OS's shell configuration environment
pub struct InterimLispVM {
    pub variables: Vec<(String, LispVal)>,
}

impl InterimLispVM {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    /// Evaluates a Lisp expression inside the VM environment
    pub fn eval(&mut self, val: &LispVal) -> Result<LispVal, &'static str> {
        match val {
            LispVal::Number(n) => Ok(LispVal::Number(*n)),
            LispVal::Atom(s) => {
                // Lookup variable in the association list
                let mut found = None;
                for (name, value) in &self.variables {
                    if name == s {
                        found = Some(value.clone());
                        break;
                    }
                }
                if let Some(v) = found {
                    Ok(v)
                } else {
                    Ok(LispVal::Atom(s.clone()))
                }
            }
            LispVal::List(list) => {
                if list.is_empty() {
                    return Ok(LispVal::List(Vec::new()));
                }

                // First item is the operator
                match &list[0] {
                    LispVal::Atom(op) => match op.as_str() {
                        "define" => {
                            if list.len() != 3 {
                                return Err("Lisp: define expects exactly 2 arguments");
                            }
                            if let LispVal::Atom(var_name) = &list[1] {
                                let evaluated_val = self.eval(&list[2])?;
                                // Update or insert in association list
                                let mut index = None;
                                for i in 0..self.variables.len() {
                                    if &self.variables[i].0 == var_name {
                                        index = Some(i);
                                        break;
                                    }
                                }
                                if let Some(idx) = index {
                                    self.variables[idx].1 = evaluated_val.clone();
                                } else {
                                    self.variables
                                        .push((var_name.clone(), evaluated_val.clone()));
                                }
                                Ok(evaluated_val)
                            } else {
                                Err("Lisp: define target must be an atom name")
                            }
                        }
                        "+" | "add" => {
                            let mut sum = 0;
                            for arg in list.iter().skip(1) {
                                match self.eval(arg)? {
                                    LispVal::Number(n) => sum += n,
                                    _ => return Err("Lisp: add expects numeric parameters"),
                                }
                            }
                            Ok(LispVal::Number(sum))
                        }
                        "-" | "sub" => {
                            if list.len() < 2 {
                                return Err("Lisp: sub expects at least 1 parameter");
                            }
                            let mut diff = match self.eval(&list[1])? {
                                LispVal::Number(n) => n,
                                _ => return Err("Lisp: sub expects numeric parameters"),
                            };
                            if list.len() == 2 {
                                return Ok(LispVal::Number(-diff));
                            }
                            for arg in list.iter().skip(2) {
                                match self.eval(arg)? {
                                    LispVal::Number(n) => diff -= n,
                                    _ => return Err("Lisp: sub expects numeric parameters"),
                                }
                            }
                            Ok(LispVal::Number(diff))
                        }
                        _ => Err("Lisp: Unknown primitive operator"),
                    },
                    _ => Err("Lisp: Operator must be an atom identifier"),
                }
            }
        }
    }
}

impl Default for InterimLispVM {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// 2. MNT REFORM LPC TELEMETRY DRIVER
// ==========================================

/// Telemetry metrics representing the MNT Reform laptop power motherboard
pub struct ReformPowerStats {
    pub cell_voltages: [f32; 8],
    pub temperature_c: f32,
    pub current_ma: f32,
    pub capacity_pct: u8,
}

/// Simulated LPC bus driver for actual MNT Reform laptop silicon
pub struct MntReformLpcDriver {
    pub backlight_level: AtomicUsize,
    pub trackball_led_color: [u8; 3],
}

impl MntReformLpcDriver {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            backlight_level: AtomicUsize::new(80),
            trackball_led_color: [0, 120, 255],
        }
    }

    /// Retrieve battery telemetry stats from the LPC bus
    pub fn query_power_stats(&self) -> ReformPowerStats {
        ReformPowerStats {
            cell_voltages: [3.35, 3.32, 3.34, 3.33, 3.31, 3.36, 3.33, 3.34],
            temperature_c: 28.5,
            current_ma: -450.0, // Active discharge
            capacity_pct: 88,
        }
    }

    /// Set Reform keyboard backlight intensity
    pub fn set_backlight_intensity(&self, level: usize) -> Result<(), &'static str> {
        if level > 100 {
            return Err("Backlight level must be between 0 and 100%");
        }
        self.backlight_level.store(level, Ordering::SeqCst);
        Ok(())
    }

    /// Set Reform trackball RGB breathing LED color
    pub fn set_trackball_color(&mut self, r: u8, g: u8, b: u8) {
        self.trackball_led_color = [r, g, b];
    }
}

impl Default for MntReformLpcDriver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interim_lisp_evaluation() {
        let mut vm = InterimLispVM::new();

        // 1. Evaluate Number -> 42
        let val_num = LispVal::Number(42);
        assert_eq!(vm.eval(&val_num).unwrap(), LispVal::Number(42));

        // 2. Evaluate (define my-val (+ 10 20))
        let define_expr = LispVal::List(vec![
            LispVal::Atom(String::from("define")),
            LispVal::Atom(String::from("my-val")),
            LispVal::List(vec![
                LispVal::Atom(String::from("add")),
                LispVal::Number(10),
                LispVal::Number(20),
            ]),
        ]);

        let res = vm.eval(&define_expr).unwrap();
        assert_eq!(res, LispVal::Number(30));

        let mut my_val = None;
        for (name, val) in &vm.variables {
            if name == "my-val" {
                my_val = Some(val.clone());
                break;
            }
        }
        assert_eq!(my_val.unwrap(), LispVal::Number(30));

        // 3. Evaluate (- my-val 5) -> 25
        let sub_expr = LispVal::List(vec![
            LispVal::Atom(String::from("sub")),
            LispVal::Atom(String::from("my-val")),
            LispVal::Number(5),
        ]);
        assert_eq!(vm.eval(&sub_expr).unwrap(), LispVal::Number(25));
    }

    #[test]
    fn test_mnt_reform_driver_telemetry() {
        let mut driver = MntReformLpcDriver::new();
        let stats = driver.query_power_stats();

        assert_eq!(stats.capacity_pct, 88);
        assert_eq!(stats.cell_voltages[0], 3.35);

        // Modify backlight intensity
        assert!(driver.set_backlight_intensity(95).is_ok());
        assert_eq!(driver.backlight_level.load(Ordering::SeqCst), 95);

        // Modify trackball RGB LED
        driver.set_trackball_color(255, 0, 0);
        assert_eq!(driver.trackball_led_color, [255, 0, 0]);
    }
}
