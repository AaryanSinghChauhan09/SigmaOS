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
    String(String),
    Char(char),
    Bool(bool),
    List(Vec<LispVal>),
}

/// S-Expression Lisp VM mimicking Interim OS's shell configuration environment
pub struct InterimLispVM {
    pub variables: Vec<(String, LispVal)>,
}

impl InterimLispVM {
    pub fn new() -> Self {
        Self {
            variables: Vec::new(),
        }
    }

    pub fn set_var(&mut self, name: String, val: LispVal) {
        let mut index = None;
        for i in 0..self.variables.len() {
            if self.variables[i].0 == name {
                index = Some(i);
                break;
            }
        }
        if let Some(idx) = index {
            self.variables[idx].1 = val;
        } else {
            self.variables.push((name, val));
        }
    }

    /// Evaluates a Lisp expression inside the VM environment
    pub fn eval(&mut self, val: &LispVal) -> Result<LispVal, &'static str> {
        match val {
            LispVal::Number(n) => Ok(LispVal::Number(*n)),
            LispVal::String(s) => Ok(LispVal::String(s.clone())),
            LispVal::Char(c) => Ok(LispVal::Char(*c)),
            LispVal::Bool(b) => Ok(LispVal::Bool(*b)),
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
                                self.set_var(var_name.clone(), evaluated_val.clone());
                                Ok(evaluated_val)
                            } else {
                                Err("Lisp: define target must be an atom name")
                            }
                        }
                        "begin" => {
                            let mut last = LispVal::Bool(false);
                            for arg in list.iter().skip(1) {
                                last = self.eval(arg)?;
                            }
                            Ok(last)
                        }
                        "if" => {
                            if list.len() != 4 {
                                return Err("Lisp: if expects condition, then, and else expressions");
                            }
                            let cond_val = self.eval(&list[1])?;
                            let cond_bool = match cond_val {
                                LispVal::Bool(b) => b,
                                LispVal::Number(n) => n != 0,
                                _ => true,
                            };
                            if cond_bool {
                                self.eval(&list[2])
                            } else {
                                self.eval(&list[3])
                            }
                        }
                        "while" => {
                            if list.len() < 3 {
                                return Err("Lisp: while expects condition and body expressions");
                            }
                            let mut last = LispVal::Bool(false);
                            loop {
                                let cond_val = self.eval(&list[1])?;
                                let cond_bool = match cond_val {
                                    LispVal::Bool(b) => b,
                                    LispVal::Number(n) => n != 0,
                                    _ => true,
                                };
                                if !cond_bool {
                                    break;
                                }
                                for arg in list.iter().skip(2) {
                                    last = self.eval(arg)?;
                                }
                            }
                            Ok(last)
                        }
                        "for" => {
                            if list.len() < 5 {
                                return Err("Lisp: for expects variable, start, end, and body expressions");
                            }
                            if let LispVal::Atom(var_name) = &list[1] {
                                let start_val = match self.eval(&list[2])? {
                                    LispVal::Number(n) => n,
                                    _ => return Err("Lisp: for expects numeric start value"),
                                };
                                let end_val = match self.eval(&list[3])? {
                                    LispVal::Number(n) => n,
                                    _ => return Err("Lisp: for expects numeric end value"),
                                };

                                let mut last = LispVal::Bool(false);
                                for val in start_val..end_val {
                                    self.set_var(var_name.clone(), LispVal::Number(val));
                                    for arg in list.iter().skip(4) {
                                        last = self.eval(arg)?;
                                    }
                                }
                                Ok(last)
                            } else {
                                Err("Lisp: for variable must be an atom name")
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
                        "<" => {
                            if list.len() != 3 {
                                return Err("Lisp: < expects exactly 2 arguments");
                            }
                            let left = match self.eval(&list[1])? {
                                LispVal::Number(n) => n,
                                _ => return Err("Lisp: < expects numeric parameters"),
                            };
                            let right = match self.eval(&list[2])? {
                                LispVal::Number(n) => n,
                                _ => return Err("Lisp: < expects numeric parameters"),
                            };
                            Ok(LispVal::Bool(left < right))
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
    fn test_lisp_advanced_loops_blocks_and_conditionals() {
        let mut vm = InterimLispVM::new();

        // Test LispVal string and char types
        let val_str = LispVal::String(String::from("SigmaOS"));
        let val_char = LispVal::Char('Σ');
        assert_eq!(vm.eval(&val_str).unwrap(), LispVal::String(String::from("SigmaOS")));
        assert_eq!(vm.eval(&val_char).unwrap(), LispVal::Char('Σ'));

        // Test begin block
        let begin_expr = LispVal::List(vec![
            LispVal::Atom(String::from("begin")),
            LispVal::Number(1),
            LispVal::Number(2),
            LispVal::Number(3),
        ]);
        assert_eq!(vm.eval(&begin_expr).unwrap(), LispVal::Number(3));

        // Test if-else conditional
        let if_expr = LispVal::List(vec![
            LispVal::Atom(String::from("if")),
            LispVal::Bool(true),
            LispVal::Number(100),
            LispVal::Number(200),
        ]);
        assert_eq!(vm.eval(&if_expr).unwrap(), LispVal::Number(100));

        // Test for loop: (for i 1 5 (define sum (+ sum i)))
        vm.set_var(String::from("sum"), LispVal::Number(0));
        let for_expr = LispVal::List(vec![
            LispVal::Atom(String::from("for")),
            LispVal::Atom(String::from("i")),
            LispVal::Number(1),
            LispVal::Number(5),
            LispVal::List(vec![
                LispVal::Atom(String::from("define")),
                LispVal::Atom(String::from("sum")),
                LispVal::List(vec![
                    LispVal::Atom(String::from("add")),
                    LispVal::Atom(String::from("sum")),
                    LispVal::Atom(String::from("i")),
                ]),
            ]),
        ]);
        vm.eval(&for_expr).unwrap();
        // 1 + 2 + 3 + 4 = 10
        assert_eq!(vm.eval(&LispVal::Atom(String::from("sum"))).unwrap(), LispVal::Number(10));

        // Test while loop: (while (< count 3) (define count (+ count 1)))
        vm.set_var(String::from("count"), LispVal::Number(0));
        let while_expr = LispVal::List(vec![
            LispVal::Atom(String::from("while")),
            LispVal::List(vec![
                LispVal::Atom(String::from("<")),
                LispVal::Atom(String::from("count")),
                LispVal::Number(3),
            ]),
            LispVal::List(vec![
                LispVal::Atom(String::from("define")),
                LispVal::Atom(String::from("count")),
                LispVal::List(vec![
                    LispVal::Atom(String::from("add")),
                    LispVal::Atom(String::from("count")),
                    LispVal::Number(1),
                ]),
            ]),
        ]);
        vm.eval(&while_expr).unwrap();
        assert_eq!(vm.eval(&LispVal::Atom(String::from("count"))).unwrap(), LispVal::Number(3));
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
