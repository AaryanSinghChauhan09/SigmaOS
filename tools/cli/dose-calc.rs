// SPDX-License-Identifier: GPL-2.0-or-later
//! dose-calc — SigmaOS Clinical Dosage Calculator
//!
//! Evidence-based drug dose calculator for healthcare professionals.
//! Implements weight-based dosing, renal/hepatic adjustment, and
//! BSA-based oncology protocols. All formulae are read-only reference tools
//! — final clinical decisions remain with the licensed practitioner.
//!
//! Usage:
//!   dose-calc <drug|list|renal|hepatic|bsa|creatinine|ideal-bw> [options]

use std::env;
use std::process::exit;

const VERSION: &str = "1.0.0";

fn cyan(s: &str)   -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str)  -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn red(s: &str)    -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str) -> String { format!("\x1B[1;33m{}\x1B[0m", s) }
fn bold(s: &str)   -> String { format!("\x1B[1m{}\x1B[0m", s) }
fn dim(s: &str)    -> String { format!("\x1B[2m{}\x1B[0m", s) }

fn print_usage() {
    println!("{} v{}", cyan("dose-calc"), VERSION);
    println!("{}", dim("⚠  Reference tool only — clinical decisions require licensed practitioner judgement."));
    println!();
    println!("{}  dose-calc <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("COMMANDS:"));
    println!("  drug        <name> --weight <kg>           Weight-based dosing");
    println!("  list        [--category <cat>]             List drug database");
    println!("  renal       --egfr <ml/min> --drug <name>  Renal dose adjustment");
    println!("  hepatic     --class <A|B|C> --drug <name>  Hepatic dose adjustment");
    println!("  bsa         --height <cm> --weight <kg>    Body surface area (Mosteller)");
    println!("  creatinine  --age <y> --weight <kg> --scr <mg/dL> [--female]  CrCl (Cockcroft-Gault)");
    println!("  ideal-bw    --height <cm> [--female]       Ideal body weight");
    println!("  aki         --baseline <mg/dL> --current <mg/dL>  AKI staging");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --weight <kg>    Patient weight in kg");
    println!("  --height <cm>    Patient height in cm");
    println!("  --age    <y>     Patient age in years");
    println!("  --female         Female patient (affects IBW, CrCl)");
    println!("  --egfr   <n>     Estimated GFR in ml/min/1.73m²");
    println!("  --scr    <n>     Serum creatinine in mg/dL");
    println!("  --drug   <name>  Drug name for dose adjustment");
    println!("  --class  <A|B|C> Child-Pugh hepatic class");
    println!("  --json           Machine-readable JSON output");
    println!("  --version, -V    Print version");
    println!("  --help,    -h    Show this help");
}

// ─── Drug database ────────────────────────────────────────────────────────────
struct Drug {
    name:         &'static str,
    category:     &'static str,
    dose_mg_kg:   f64,
    max_dose_mg:  f64,
    unit:         &'static str,
    frequency:    &'static str,
    route:        &'static str,
    renal_adj:    bool,
    hepatic_adj:  bool,
    note:         &'static str,
}

fn drug_db() -> Vec<Drug> {
    vec![
        Drug { name:"amoxicillin",     category:"antibiotic",   dose_mg_kg:25.0,  max_dose_mg:500.0,  unit:"mg",   frequency:"q8h",     route:"PO",  renal_adj:true,  hepatic_adj:false, note:"Standard adult dose 500mg q8h" },
        Drug { name:"amoxicillin-clav", category:"antibiotic",  dose_mg_kg:22.5,  max_dose_mg:875.0,  unit:"mg",   frequency:"q12h",    route:"PO",  renal_adj:true,  hepatic_adj:false, note:"Augmentin; base on amoxicillin component" },
        Drug { name:"azithromycin",    category:"antibiotic",   dose_mg_kg:10.0,  max_dose_mg:500.0,  unit:"mg",   frequency:"q24h",    route:"PO",  renal_adj:false, hepatic_adj:true,  note:"Day 1 load; days 2-5 half dose" },
        Drug { name:"ciprofloxacin",   category:"antibiotic",   dose_mg_kg:10.0,  max_dose_mg:500.0,  unit:"mg",   frequency:"q12h",    route:"PO",  renal_adj:true,  hepatic_adj:false, note:"Avoid in paeds <18y; adjust for GFR<30" },
        Drug { name:"metformin",       category:"antidiabetic", dose_mg_kg:0.0,   max_dose_mg:1000.0, unit:"mg",   frequency:"q12h",    route:"PO",  renal_adj:true,  hepatic_adj:false, note:"Fixed dose 500-1000mg; CI if eGFR<30" },
        Drug { name:"paracetamol",     category:"analgesic",    dose_mg_kg:15.0,  max_dose_mg:1000.0, unit:"mg",   frequency:"q6h",     route:"PO",  renal_adj:false, hepatic_adj:true,  note:"Max 4g/day adult; reduce in hepatic impairment" },
        Drug { name:"ibuprofen",       category:"nsaid",        dose_mg_kg:10.0,  max_dose_mg:400.0,  unit:"mg",   frequency:"q8h",     route:"PO",  renal_adj:true,  hepatic_adj:true,  note:"Use with food; avoid in renal impairment" },
        Drug { name:"gentamicin",      category:"antibiotic",   dose_mg_kg:5.0,   max_dose_mg:360.0,  unit:"mg",   frequency:"q24h",    route:"IV",  renal_adj:true,  hepatic_adj:false, note:"Monitor trough <1 mg/L; nephrotoxic" },
        Drug { name:"vancomycin",      category:"antibiotic",   dose_mg_kg:15.0,  max_dose_mg:2000.0, unit:"mg",   frequency:"q12h",    route:"IV",  renal_adj:true,  hepatic_adj:false, note:"Target AUC 400-600; TDM required" },
        Drug { name:"insulin-regular", category:"insulin",      dose_mg_kg:0.0,   max_dose_mg:0.0,    unit:"units",frequency:"variable", route:"SC",  renal_adj:true,  hepatic_adj:false, note:"0.1 units/kg for hyperglycaemia; individualise" },
        Drug { name:"amlodipine",      category:"antihypert",   dose_mg_kg:0.0,   max_dose_mg:10.0,   unit:"mg",   frequency:"q24h",    route:"PO",  renal_adj:false, hepatic_adj:true,  note:"Fixed dose 5-10mg; titrate to BP response" },
        Drug { name:"furosemide",      category:"diuretic",     dose_mg_kg:0.5,   max_dose_mg:80.0,   unit:"mg",   frequency:"q12h",    route:"PO",  renal_adj:true,  hepatic_adj:false, note:"May need higher doses in renal impairment" },
        Drug { name:"dexamethasone",   category:"steroid",      dose_mg_kg:0.15,  max_dose_mg:20.0,   unit:"mg",   frequency:"q6h",     route:"IV",  renal_adj:false, hepatic_adj:true,  note:"COVID: 6mg q24h; anti-emetic: 4-8mg" },
        Drug { name:"atorvastatin",    category:"statin",       dose_mg_kg:0.0,   max_dose_mg:80.0,   unit:"mg",   frequency:"q24h",    route:"PO",  renal_adj:false, hepatic_adj:true,  note:"Fixed dose 10-80mg; CI in active liver disease" },
        Drug { name:"oseltamivir",     category:"antiviral",    dose_mg_kg:0.0,   max_dose_mg:75.0,   unit:"mg",   frequency:"q12h",    route:"PO",  renal_adj:true,  hepatic_adj:false, note:"Fixed 75mg q12h; adjust if GFR<30" },
    ]
}

// ─── Calculations ──────────────────────────────────────────────────────────

fn ideal_body_weight(height_cm: f64, female: bool) -> f64 {
    let h_in = height_cm / 2.54;
    let base = if female { 45.5 } else { 50.0 };
    (base + 2.3 * (h_in - 60.0)).max(0.0)
}

fn bsa_mosteller(height_cm: f64, weight_kg: f64) -> f64 {
    ((height_cm * weight_kg) / 3600.0_f64).sqrt()
}

fn egfr_cockcroft_gault(age: f64, weight_kg: f64, scr: f64, female: bool) -> f64 {
    let sex_factor = if female { 0.85 } else { 1.0 };
    ((140.0 - age) * weight_kg * sex_factor) / (72.0 * scr)
}

fn renal_adjustment(egfr: f64, drug: &str) -> (&'static str, f64) {
    // Returns (recommendation, dose_factor)
    match drug {
        "ciprofloxacin" => {
            if egfr >= 30.0 { ("Normal dose", 1.0) }
            else if egfr >= 15.0 { ("250-500mg q12-24h", 0.5) }
            else { ("250mg q24h or avoid", 0.25) }
        }
        "metformin" => {
            if egfr >= 45.0 { ("Normal dose", 1.0) }
            else if egfr >= 30.0 { ("Use with caution; reduce dose", 0.5) }
            else { ("Contraindicated", 0.0) }
        }
        "gentamicin" | "vancomycin" => {
            if egfr >= 60.0 { ("Normal dose; TDM required", 1.0) }
            else if egfr >= 30.0 { ("Extend interval to q36-48h; TDM", 0.7) }
            else { ("Extended interval q48-72h; TDM essential", 0.5) }
        }
        "furosemide" => {
            if egfr >= 30.0 { ("Normal dose", 1.0) }
            else { ("May need 2-4× normal dose for effect", 2.0) }
        }
        "oseltamivir" => {
            if egfr >= 30.0 { ("75mg q12h", 1.0) }
            else { ("75mg q24h", 0.5) }
        }
        _ => {
            if egfr >= 60.0 { ("Normal dose", 1.0) }
            else if egfr >= 30.0 { ("Review; consider 75% of normal dose", 0.75) }
            else { ("Significant reduction required; specialist review", 0.5) }
        }
    }
}

fn hepatic_adjustment(child_pugh: &str, drug: &str) -> (&'static str, f64) {
    match child_pugh.to_uppercase().as_str() {
        "A" => ("Normal dose; monitor LFTs", 1.0),
        "B" => match drug {
            "paracetamol"   => ("Max 2g/day; avoid if possible", 0.5),
            "azithromycin"  => ("Use with caution; max 250mg", 0.5),
            "atorvastatin"  => ("Contraindicated", 0.0),
            "amlodipine"    => ("Start 2.5mg; titrate slowly", 0.5),
            "dexamethasone" => ("Reduce by 25%; monitor", 0.75),
            _ => ("Reduce by 25-50%; monitor LFTs", 0.5),
        },
        "C" => match drug {
            "paracetamol"   => ("Avoid", 0.0),
            "azithromycin"  => ("Avoid", 0.0),
            "atorvastatin"  => ("Contraindicated", 0.0),
            _ => ("Major reduction or avoid; specialist consult required", 0.25),
        },
        _ => ("Unknown class; consult specialist", 1.0),
    }
}

fn aki_stage(baseline: f64, current: f64) -> (&'static str, &'static str) {
    let ratio = current / baseline;
    let rise  = current - baseline;
    if ratio >= 3.0 || current >= 4.0 {
        ("Stage 3 (Severe)", "Nephrology consult; consider RRT")
    } else if ratio >= 2.0 || rise >= 1.5 {
        ("Stage 2 (Moderate)", "Daily renal monitoring; optimise fluid balance")
    } else if ratio >= 1.5 || rise >= 0.3 {
        ("Stage 1 (Mild)", "Monitor creatinine q24h; identify and treat cause")
    } else {
        ("No AKI criteria met", "Continue monitoring per clinical judgement")
    }
}

// ─── Command handlers ──────────────────────────────────────────────────────

fn cmd_drug(name: &str, weight: f64, json: bool) {
    let db = drug_db();
    let drug = db.iter().find(|d| d.name.eq_ignore_ascii_case(name));
    match drug {
        None => {
            eprintln!("{} Drug '{}' not in database. Run 'dose-calc list'.", red("error:"), name);
        }
        Some(d) => {
            let dose = if d.dose_mg_kg > 0.0 {
                (d.dose_mg_kg * weight).min(d.max_dose_mg)
            } else { d.max_dose_mg };
            if json {
                println!("{{\"drug\":\"{}\",\"dose\":{:.1},\"unit\":\"{}\",\"frequency\":\"{}\",\"route\":\"{}\"}}",
                    d.name, dose, d.unit, d.frequency, d.route);
                return;
            }
            println!("{}", bold(&format!("Dose: {}", d.name)));
            println!("  Patient weight : {:.1} kg", weight);
            println!("  Dose           : {} {:.1} {} ({} mg/kg × {:.1} kg)",
                green(&format!("{:.1}", dose)), dose, d.unit,
                d.dose_mg_kg, weight);
            println!("  Frequency      : {}", d.frequency);
            println!("  Route          : {}", d.route);
            if d.dose_mg_kg > 0.0 {
                println!("  Formula        : {:.1} mg/kg × {:.1} kg = {:.1} {} (max {:.0} {})",
                    d.dose_mg_kg, weight, dose, d.unit, d.max_dose_mg, d.unit);
            }
            println!("  Note           : {}", dim(d.note));
            if d.renal_adj  { println!("  {}  Requires renal dose adjustment",   yellow("⚠")); }
            if d.hepatic_adj { println!("  {}  Requires hepatic dose adjustment", yellow("⚠")); }
            println!("\n  {}", dim("⚠  Clinical tool — verify with current formulary and patient-specific factors."));
        }
    }
}

fn cmd_list(category: Option<&str>, json: bool) {
    let db = drug_db();
    let visible: Vec<&Drug> = db.iter()
        .filter(|d| category.map_or(true, |c| d.category.eq_ignore_ascii_case(c)))
        .collect();
    if json {
        println!("[{}]", visible.iter().map(|d|
            format!("{{\"name\":\"{}\",\"category\":\"{}\",\"route\":\"{}\",\"frequency\":\"{}\"}}",
                d.name, d.category, d.route, d.frequency)
        ).collect::<Vec<_>>().join(","));
        return;
    }
    println!("{} ({} drugs{})", bold("Drug Database"), visible.len(),
        category.map(|c| format!(", category: {}", c)).unwrap_or_default());
    println!("  {:<22}  {:<14}  {:<8}  {:<8}  {}", "Drug", "Category", "Route", "Frequency", "Dose");
    println!("  {}", "─".repeat(78));
    for d in &visible {
        let dose_str = if d.dose_mg_kg > 0.0 { format!("{:.1} mg/kg", d.dose_mg_kg) }
                       else { format!("{:.0} mg fixed", d.max_dose_mg) };
        println!("  {:<22}  {:<14}  {:<8}  {:<8}  {}", d.name, d.category, d.route, d.frequency, dose_str);
    }
}

fn cmd_bsa(height: f64, weight: f64, json: bool) {
    let bsa = bsa_mosteller(height, weight);
    if json { println!("{{\"bsa_m2\":{:.2},\"formula\":\"Mosteller\"}}", bsa); return; }
    println!("{} (Mosteller formula)", bold("Body Surface Area"));
    println!("  Height : {:.1} cm", height);
    println!("  Weight : {:.1} kg", weight);
    println!("  BSA    : {} m²", green(&format!("{:.2}", bsa)));
    println!("  {}  BSA = √(height_cm × weight_kg / 3600)", dim("Formula:"));
}

fn cmd_creatinine(age: f64, weight: f64, scr: f64, female: bool, json: bool) {
    let crcl = egfr_cockcroft_gault(age, weight, scr, female);
    if json {
        println!("{{\"crcl_ml_min\":{:.1},\"egfr_adj\":{:.1},\"formula\":\"Cockcroft-Gault\"}}",
            crcl, crcl);
        return;
    }
    let sex = if female { "Female" } else { "Male" };
    let category = if crcl >= 90.0 { green("Normal") }
                   else if crcl >= 60.0 { green("Mildly reduced (G2)") }
                   else if crcl >= 45.0 { yellow("Moderately reduced (G3a)") }
                   else if crcl >= 30.0 { yellow("Mod-severely reduced (G3b)") }
                   else if crcl >= 15.0 { red("Severely reduced (G4)") }
                   else { red("Kidney failure (G5)") };
    println!("{} (Cockcroft-Gault)", bold("Creatinine Clearance"));
    println!("  Age    : {:.0}y  Weight : {:.1}kg  SCr : {:.2} mg/dL  Sex : {}", age, weight, scr, sex);
    println!("  CrCl   : {} ml/min", green(&format!("{:.1}", crcl)));
    println!("  eGFR   : {}", category);
    println!("  {}  (140 - age) × weight × {} / (72 × SCr)", dim("Formula:"), if female { "0.85" } else { "1.00" });
}

fn cmd_ideal_bw(height: f64, female: bool, json: bool) {
    let ibw = ideal_body_weight(height, female);
    if json { println!("{{\"ibw_kg\":{:.1},\"formula\":\"Devine\"}}", ibw); return; }
    println!("{}", bold("Ideal Body Weight (Devine)"));
    println!("  Height : {:.1} cm  Sex : {}", height, if female { "Female" } else { "Male" });
    println!("  IBW    : {} kg", green(&format!("{:.1}", ibw)));
    println!("  {}  {} + 2.3 × (height_inches - 60)", dim("Formula:"), if female { "45.5" } else { "50.0" });
}

fn cmd_renal(egfr: f64, drug: &str, json: bool) {
    let (rec, factor) = renal_adjustment(egfr, drug);
    if json {
        println!("{{\"drug\":\"{}\",\"egfr\":{:.1},\"recommendation\":\"{}\",\"dose_factor\":{:.2}}}",
            drug, egfr, rec, factor);
        return;
    }
    let stage = if egfr >= 90.0 { green("Normal (G1)") }
                else if egfr >= 60.0 { green("G2") }
                else if egfr >= 30.0 { yellow("G3") }
                else if egfr >= 15.0 { red("G4") }
                else { red("G5/ESRD") };
    println!("{}", bold("Renal Dose Adjustment"));
    println!("  Drug       : {}", drug);
    println!("  eGFR       : {:.1} ml/min  [{}]", egfr, stage);
    println!("  Adjustment : {}", if factor < 0.5 { red(rec) } else if factor < 1.0 { yellow(rec) } else { green(rec) });
    if factor == 0.0 { println!("  {} Contraindicated in this degree of renal impairment.", red("⚠")); }
}

fn cmd_hepatic(child_pugh: &str, drug: &str, json: bool) {
    let (rec, factor) = hepatic_adjustment(child_pugh, drug);
    if json {
        println!("{{\"drug\":\"{}\",\"child_pugh\":\"{}\",\"recommendation\":\"{}\",\"dose_factor\":{:.2}}}",
            drug, child_pugh, rec, factor);
        return;
    }
    println!("{}", bold("Hepatic Dose Adjustment"));
    println!("  Drug        : {}", drug);
    println!("  Child-Pugh  : Class {}", child_pugh.to_uppercase());
    println!("  Adjustment  : {}", if factor == 0.0 { red(rec) } else if factor < 1.0 { yellow(rec) } else { green(rec) });
    if factor == 0.0 { println!("  {} Avoid in this hepatic impairment class.", red("⚠")); }
}

fn cmd_aki(baseline: f64, current: f64, json: bool) {
    let (stage, action) = aki_stage(baseline, current);
    let ratio = current / baseline;
    let rise  = current - baseline;
    if json {
        println!("{{\"baseline\":{:.2},\"current\":{:.2},\"ratio\":{:.2},\"rise\":{:.2},\"stage\":\"{}\"}}",
            baseline, current, ratio, rise, stage);
        return;
    }
    println!("{}", bold("AKI Staging (KDIGO 2012)"));
    println!("  Baseline SCr : {:.2} mg/dL", baseline);
    println!("  Current SCr  : {:.2} mg/dL", current);
    println!("  Ratio        : {:.2}×  Rise: +{:.2} mg/dL", ratio, rise);
    let stage_col = if stage.contains("Stage 3") { red(stage) }
                    else if stage.contains("Stage 2") || stage.contains("Stage 1") { yellow(stage) }
                    else { green(stage) };
    println!("  Stage        : {}", stage_col);
    println!("  Action       : {}", action);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" { print_usage(); exit(if args.len()<2{1}else{0}); }
    if args[1] == "--version" || args[1] == "-V" { println!("dose-calc {}", VERSION); exit(0); }

    let json    = args.iter().any(|a| a == "--json");
    let female  = args.iter().any(|a| a == "--female");
    let weight  = args.windows(2).find(|w| w[0]=="--weight").and_then(|w| w[1].parse().ok()).unwrap_or(70.0f64);
    let height  = args.windows(2).find(|w| w[0]=="--height").and_then(|w| w[1].parse().ok()).unwrap_or(170.0f64);
    let age     = args.windows(2).find(|w| w[0]=="--age").and_then(|w| w[1].parse().ok()).unwrap_or(50.0f64);
    let egfr    = args.windows(2).find(|w| w[0]=="--egfr").and_then(|w| w[1].parse().ok()).unwrap_or(90.0f64);
    let scr     = args.windows(2).find(|w| w[0]=="--scr").and_then(|w| w[1].parse().ok()).unwrap_or(1.0f64);
    let drug    = args.windows(2).find(|w| w[0]=="--drug").map(|w| w[1].as_str()).unwrap_or("paracetamol");
    let class_  = args.windows(2).find(|w| w[0]=="--class").map(|w| w[1].as_str()).unwrap_or("A");
    let cat     = args.windows(2).find(|w| w[0]=="--category").map(|w| w[1].as_str());
    let bscr    = args.windows(2).find(|w| w[0]=="--baseline").and_then(|w| w[1].parse().ok()).unwrap_or(0.9f64);
    let cscr    = args.windows(2).find(|w| w[0]=="--current").and_then(|w| w[1].parse().ok()).unwrap_or(1.5f64);

    let positional: Vec<&str> = args[2..].iter()
        .filter(|a| !a.starts_with("--"))
        .map(|s| s.as_str()).collect();

    match args[1].as_str() {
        "drug"        => cmd_drug(positional.first().copied().unwrap_or(drug), weight, json),
        "list"        => cmd_list(cat, json),
        "bsa"         => cmd_bsa(height, weight, json),
        "creatinine"  => cmd_creatinine(age, weight, scr, female, json),
        "ideal-bw"    => cmd_ideal_bw(height, female, json),
        "renal"       => cmd_renal(egfr, drug, json),
        "hepatic"     => cmd_hepatic(class_, drug, json),
        "aki"         => cmd_aki(bscr, cscr, json),
        _ => { eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]); exit(1); }
    }
}
