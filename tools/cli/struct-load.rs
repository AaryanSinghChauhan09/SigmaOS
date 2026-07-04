// SPDX-License-Identifier: GPL-2.0-or-later
//! struct-load — Structural Load Analysis CLI
//!
//! Civil/structural engineering tool for beam, column, slab, and foundation
//! load calculations per IS 456:2000 (Reinforced Concrete) and IS 800:2007 (Steel).
//!
//! Usage:
//!   struct-load <beam|column|slab|foundation|wind|seismic|combo> [options]

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
    println!("{} v{}  — Structural Load Analysis (IS 456/800)", cyan("struct-load"), VERSION);
    println!();
    println!("{}  struct-load <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("COMMANDS:"));
    println!("  beam        Calculate simply-supported or cantilever beam reactions/moments");
    println!("  column      Axial + eccentric column load capacity");
    println!("  slab        Two-way slab (IS 456) design moments");
    println!("  foundation  Isolated footing bearing pressure");
    println!("  wind        Basic wind pressure (IS 875 Part 3)");
    println!("  seismic     Seismic base shear (IS 1893)");
    println!("  combo       Load combination per IS 456 Table 18");
    println!("  section     Section properties (I, Z, r) for common profiles");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --span  <m>       Beam/slab span in metres");
    println!("  --udl   <kN/m>    Uniformly distributed load");
    println!("  --pl    <kN>      Point load");
    println!("  --width <m>       Width (column/footing)");
    println!("  --depth <m>       Depth (beam/column/footing)");
    println!("  --height <m>      Column or structure height");
    println!("  --fck   <MPa>     Concrete characteristic strength (default: 25)");
    println!("  --fy    <MPa>     Steel yield strength (default: 415)");
    println!("  --dl    <kN/m²>   Dead load intensity");
    println!("  --ll    <kN/m²>   Live load intensity");
    println!("  --wind  <kN/m²>   Wind pressure");
    println!("  --zone  <II-V>    Seismic zone (IS 1893)");
    println!("  --cantilever      Use cantilever instead of simply-supported");
    println!("  --json            Machine-readable JSON output");
    println!("  --version, -V     Print version");
    println!("  --help,    -h     Show this help");
}

// ─── Helpers ──────────────────────────────────────────────────────────────────

fn is_456_load_factor_dl() -> f64 { 1.5 }
fn is_456_load_factor_ll() -> f64 { 1.5 }
fn is_456_load_factor_wl() -> f64 { 1.5 }

// ─── Beam Analysis ───────────────────────────────────────────────────────────

fn cmd_beam(span: f64, udl: f64, pl: f64, cantilever: bool, json: bool) {
    if cantilever {
        let r_a     = udl * span + pl;
        let m_fixed = udl * span * span / 2.0 + pl * span;
        let shear_max = r_a;
        if json {
            println!("{{\"type\":\"cantilever\",\"span\":{},\"r_fixed\":{:.2},\"m_fixed\":{:.2},\"shear_max\":{:.2}}}",
                span, r_a, m_fixed, shear_max);
            return;
        }
        println!("{} — Cantilever", bold("Beam Analysis"));
        println!("  Span        : {:.2} m", span);
        println!("  UDL         : {:.2} kN/m", udl);
        println!("  Point Load  : {:.2} kN", pl);
        println!("  Fixed-end Rx: {} kN (↑)", green(&format!("{:.2}", r_a)));
        println!("  Fixed-end M : {} kN·m (hogging)", red(&format!("{:.2}", m_fixed)));
        println!("  Max Shear   : {:.2} kN", shear_max);
    } else {
        let total_w = udl * span;
        let r_a     = total_w / 2.0 + pl / 2.0;
        let r_b     = total_w / 2.0 + pl / 2.0;
        let m_max   = udl * span * span / 8.0 + pl * span / 4.0;
        let shear_a = r_a;
        if json {
            println!("{{\"type\":\"simply_supported\",\"span\":{},\"r_a\":{:.2},\"r_b\":{:.2},\"m_max\":{:.2},\"shear_a\":{:.2}}}",
                span, r_a, r_b, m_max, shear_a);
            return;
        }
        println!("{} — Simply Supported", bold("Beam Analysis (IS 456)"));
        println!("  Span        : {:.2} m", span);
        println!("  UDL         : {:.2} kN/m   Point Load: {:.2} kN", udl, pl);
        println!("  Reaction RA : {} kN (↑)", green(&format!("{:.2}", r_a)));
        println!("  Reaction RB : {} kN (↑)", green(&format!("{:.2}", r_b)));
        println!("  Max Moment  : {} kN·m (sagging, mid-span)", red(&format!("{:.2}", m_max)));
        println!("  Max Shear   : {:.2} kN (at supports)", shear_a);
        let mu   = m_max * 1e6;
        let f_ck = 25.0f64;
        let b    = 0.3f64; // assumed 300mm
        let d    = (mu / (0.133 * f_ck * b * 1e3)).sqrt() / 1e3;
        println!("\n  {}  IS 456 depth check (b=300mm, fck=25 MPa): d_req ≈ {:.0} mm", dim("Hint:"), d * 1000.0);
    }
}

// ─── Column Analysis ──────────────────────────────────────────────────────────

fn cmd_column(width: f64, depth: f64, height: f64, fck: f64, fy: f64, axial: f64, json: bool) {
    // IS 456 Cl. 39.3: P_u = 0.4 fck Ac + 0.67 fy Asc
    // Assume 2% steel (Asc = 0.02 * Ag)
    let ag  = width * depth * 1e6; // mm²
    let asc = 0.02 * ag;
    let ac  = ag - asc;
    let pu_capacity = (0.4 * fck * ac + 0.67 * fy * asc) / 1000.0; // kN
    let slenderness = height / (0.289 * (width.min(depth)));         // approx ke=1.0
    let util        = axial / pu_capacity;
    if json {
        println!("{{\"pu_capacity\":{:.1},\"axial\":{:.1},\"utilisation\":{:.3},\"slenderness\":{:.1}}}",
            pu_capacity, axial, util, slenderness);
        return;
    }
    println!("{}", bold("Column Analysis (IS 456 Cl. 39.3)"));
    println!("  Section     : {:.0}×{:.0} mm  Height: {:.2} m", width*1000.0, depth*1000.0, height);
    println!("  fck         : {} MPa   fy: {} MPa   Asc: 2%", fck, fy);
    println!("  Pu capacity : {} kN", green(&format!("{:.1}", pu_capacity)));
    println!("  Applied Pu  : {:.1} kN", axial);
    let util_str = format!("{:.1}%", util * 100.0);
    let util_col = if util > 1.0 { red(&util_str) } else if util > 0.85 { yellow(&util_str) } else { green(&util_str) };
    println!("  Utilisation : {}", util_col);
    println!("  Slenderness : {:.1} (limit: 12)", slenderness);
    if slenderness > 12.0 { println!("  {} Slender column — second-order effects may apply.", yellow("⚠")); }
    if util > 1.0 { println!("  {} Section is {} — increase section or reduce load.", red("✗"), red("OVERSTRESSED")); }
}

// ─── Slab Analysis ────────────────────────────────────────────────────────────

fn cmd_slab(lx: f64, ly: f64, dl: f64, ll: f64, fck: f64, fy: f64, json: bool) {
    // IS 456 Table 26 — two-way slab coefficients (simply supported, 4 edges)
    let ratio = ly / lx;
    // Approximate Bending Moment Coefficients (αx, αy) for SS slab
    let (alpha_x, alpha_y) = if ratio >= 2.0 {
        (0.125, 0.0)  // acts as one-way slab
    } else {
        let ax = 1.0 / (8.0 * (1.0 + (ratio / 2.0).powi(4)));
        let ay = ax * ratio.powi(4) / (1.0 + ratio.powi(4));
        (ax, ay)
    };

    let wu    = (1.5 * dl + 1.5 * ll);   // factored load kN/m²
    let mx    = alpha_x * wu * lx * lx;
    let my    = alpha_y * wu * lx * lx;

    if json {
        println!("{{\"lx\":{},\"ly\":{},\"wu\":{:.2},\"mx\":{:.2},\"my\":{:.2},\"ratio\":{:.2}}}",
            lx, ly, wu, mx, my, ratio);
        return;
    }
    println!("{}", bold("Two-Way Slab (IS 456)"));
    println!("  Short span lx : {:.2} m   Long span ly: {:.2} m   ly/lx: {:.2}", lx, ly, ratio);
    println!("  Dead load     : {:.1} kN/m²  Live load: {:.1} kN/m²", dl, ll);
    println!("  Factored load : {} kN/m²  (1.5DL + 1.5LL)", green(&format!("{:.2}", wu)));
    println!("  Mx (short)    : {} kN·m/m", red(&format!("{:.2}", mx)));
    println!("  My (long)     : {} kN·m/m", red(&format!("{:.2}", my)));
    let d_req = (mx * 1e6 / (0.133 * fck * 1000.0)).sqrt();
    println!("\n  {}  Min effective depth: {:.0} mm (fck={} MPa)", dim("Design hint:"), d_req, fck);
    if ratio >= 2.0 { println!("  {}  ly/lx ≥ 2 — behaves as one-way slab.", yellow("⚠")); }
}

// ─── Foundation ───────────────────────────────────────────────────────────────

fn cmd_foundation(width: f64, depth_found: f64, column_load: f64, soil_sbc: f64, json: bool) {
    // Net soil pressure = column_load / (B × L)
    let area         = width * width;   // square footing
    let self_wt      = 0.1 * column_load; // ~10% self weight
    let total_load   = column_load + self_wt;
    let gross_p      = total_load / area;
    let net_p        = column_load / area;
    let util         = gross_p / soil_sbc;
    let req_size     = (total_load / soil_sbc).sqrt();

    if json {
        println!("{{\"size\":{:.2},\"gross_p\":{:.1},\"net_p\":{:.1},\"sbc\":{:.1},\"utilisation\":{:.2}}}",
            width, gross_p, net_p, soil_sbc, util);
        return;
    }
    println!("{}", bold("Isolated Footing (IS 456)"));
    println!("  Footing size  : {:.2}×{:.2} m   Depth: {:.2} m", width, width, depth_found);
    println!("  Column load   : {:.1} kN   Self-wt: {:.1} kN", column_load, self_wt);
    println!("  Soil SBC      : {:.1} kN/m²", soil_sbc);
    println!("  Gross pressure: {} kN/m²", green(&format!("{:.1}", gross_p)));
    println!("  Net pressure  : {:.1} kN/m²", net_p);
    let util_str = format!("{:.1}%", util * 100.0);
    let util_col = if util > 1.0 { red(&util_str) } else if util > 0.9 { yellow(&util_str) } else { green(&util_str) };
    println!("  Utilisation   : {}", util_col);
    if util > 1.0 {
        println!("  {} Increase footing size to {:.2}×{:.2} m minimum.", red("✗"), req_size, req_size);
    }
}

// ─── Wind Load (IS 875 Part 3) ────────────────────────────────────────────────

fn cmd_wind(vb: f64, k1: f64, k2: f64, k3: f64, cf: f64, area: f64, json: bool) {
    let vz    = vb * k1 * k2 * k3;
    let pz    = 0.6 * vz * vz / 1000.0; // kN/m²
    let force = cf * pz * area;
    if json {
        println!("{{\"vz\":{:.1},\"pz\":{:.3},\"force\":{:.1}}}", vz, pz, force);
        return;
    }
    println!("{} (IS 875 Part 3)", bold("Wind Load"));
    println!("  Basic wind speed Vb : {:.1} m/s", vb);
    println!("  k1 (risk)           : {:.2}   k2 (terrain/height): {:.2}   k3 (topography): {:.2}", k1, k2, k3);
    println!("  Design wind speed Vz: {} m/s", green(&format!("{:.1}", vz)));
    println!("  Wind pressure pz    : {:.3} kN/m²", pz);
    println!("  Force coefficient Cf: {:.2}", cf);
    println!("  Wind force F        : {} kN", red(&format!("{:.1}", force)));
}

// ─── Seismic (IS 1893) ────────────────────────────────────────────────────────

fn cmd_seismic(w: f64, zone: &str, sa_g: f64, r: f64, i: f64, json: bool) {
    let z_factor = match zone.to_uppercase().as_str() {
        "II"  => 0.10, "III" => 0.16, "IV"  => 0.24, "V"   => 0.36, _ => 0.10,
    };
    let ah   = z_factor / 2.0 * sa_g / r * i;
    let vb   = ah * w;
    if json {
        println!("{{\"zone\":\"{}\",\"z\":{},\"ah\":{:.4},\"vb\":{:.1}}}", zone, z_factor, ah, vb);
        return;
    }
    println!("{} (IS 1893)", bold("Seismic Base Shear"));
    println!("  Seismic Zone : {}  Z = {}", zone.to_uppercase(), z_factor);
    println!("  Seismic wt W : {:.1} kN", w);
    println!("  Sa/g         : {:.2}  R: {:.1}  I: {:.1}", sa_g, r, i);
    println!("  Ah           : {:.4}  = Z/2 × Sa/g / R × I", ah);
    println!("  Base shear Vb: {} kN", red(&format!("{:.1}", vb)));
    println!("  {}  Distribute Vb over height per IS 1893 Cl. 7.7.1", dim("Note:"));
}

// ─── Load Combination (IS 456 Table 18) ──────────────────────────────────────

fn cmd_combo(dl: f64, ll: f64, wl: f64, json: bool) {
    let combos: &[(&str, f64)] = &[
        ("1.5(DL + LL)",      1.5*dl + 1.5*ll),
        ("1.2(DL + LL + WL)", 1.2*dl + 1.2*ll + 1.2*wl),
        ("1.5(DL + WL)",      1.5*dl + 1.5*wl),
        ("0.9 DL + 1.5 WL",   0.9*dl + 1.5*wl),
    ];
    let governing = combos.iter().map(|(_, v)| *v).fold(f64::NEG_INFINITY, f64::max);
    if json {
        println!("[{}]", combos.iter().map(|(n, v)|
            format!("{{\"combo\":\"{}\",\"value\":{:.2},\"governing\":{}}}", n, v, (*v - governing).abs() < 0.01)
        ).collect::<Vec<_>>().join(","));
        return;
    }
    println!("{}", bold("Load Combinations (IS 456 Table 18)"));
    println!("  DL = {:.1} kN/m²   LL = {:.1} kN/m²   WL = {:.1} kN/m²", dl, ll, wl);
    println!("  {}", "─".repeat(50));
    for (name, val) in combos {
        let marker = if (*val - governing).abs() < 0.01 { red(" ← GOVERNING") } else { String::new() };
        println!("  {:<32} {:.2} kN/m²{}", name, val, marker);
    }
}

// ─── Section properties ───────────────────────────────────────────────────────

fn cmd_section(profile: &str, json: bool) {
    let sections: &[(&str, f64, f64, f64, f64, f64)] = &[
        // (name, A mm², Ixx cm⁴, Zxx cm³, rxx cm, mass kg/m)
        ("ISMB 100", 1340.0,   257.5,  51.5,  4.38,  10.5),
        ("ISMB 150", 1840.0,  726.0,   96.8,  6.29,  14.4),
        ("ISMB 200", 3233.0, 2235.0,  223.5,  8.31,  25.4),
        ("ISMB 250", 4754.0, 5131.0,  410.5, 10.39,  37.3),
        ("ISMB 300", 6334.0, 9862.0,  657.5, 12.48,  49.7),
        ("ISMB 350", 8143.0,19158.0, 1095.0, 15.35,  63.9),
        ("ISMB 400", 7846.0,20346.0, 1017.0, 16.10,  61.6),
        ("ISMC 100", 1140.0,  187.5,   37.5,  4.06,   9.2),
        ("ISMC 150", 1898.0,  779.0,  103.9,  6.40,  14.9),
        ("ISMC 200", 2826.0, 1819.0,  181.9,  8.02,  22.1),
        ("ISA 100×100×10", 1903.0, 177.1, 25.3, 3.05, 14.9),
        ("ISA 150×150×12", 3474.0, 813.0, 74.9, 4.83, 27.3),
    ];

    let q = profile.to_ascii_uppercase();
    let matched: Vec<&&(_, f64, f64, f64, f64, f64)> = sections.iter()
        .filter(|s| s.0.to_ascii_uppercase().contains(&q)).collect();

    if json {
        println!("[{}]", matched.iter().map(|s|
            format!("{{\"section\":\"{}\",\"A_mm2\":{:.0},\"I_cm4\":{:.1},\"Z_cm3\":{:.1},\"r_cm\":{:.2},\"mass_kg_m\":{:.1}}}",
                s.0, s.1, s.2, s.3, s.4, s.5)
        ).collect::<Vec<_>>().join(","));
        return;
    }
    println!("{} — '{}' ({} match(es))", bold("Steel Section Properties (IS 808)"), profile, matched.len());
    println!("  {:<22}  {:>10}  {:>10}  {:>10}  {:>8}  {:>10}",
        "Section", "A (mm²)", "Ixx (cm⁴)", "Zxx (cm³)", "r (cm)", "Mass (kg/m)");
    println!("  {}", "─".repeat(78));
    for s in &matched {
        println!("  {:<22}  {:>10.0}  {:>10.1}  {:>10.1}  {:>8.2}  {:>10.1}",
            cyan(s.0), s.1, s.2, s.3, s.4, s.5);
    }
    if matched.is_empty() {
        println!("  No match. Try: ISMB 200, ISMC 150, ISA 100×100×10");
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" { print_usage(); exit(if args.len()<2{1}else{0}); }
    if args[1] == "--version" || args[1] == "-V" { println!("struct-load {}", VERSION); exit(0); }

    let json       = args.iter().any(|a| a == "--json");
    let cantilever = args.iter().any(|a| a == "--cantilever");
    let span   = args.windows(2).find(|w| w[0]=="--span").and_then(|w| w[1].parse().ok()).unwrap_or(5.0f64);
    let udl    = args.windows(2).find(|w| w[0]=="--udl").and_then(|w| w[1].parse().ok()).unwrap_or(10.0f64);
    let pl     = args.windows(2).find(|w| w[0]=="--pl").and_then(|w| w[1].parse().ok()).unwrap_or(0.0f64);
    let width  = args.windows(2).find(|w| w[0]=="--width").and_then(|w| w[1].parse().ok()).unwrap_or(0.3f64);
    let depth  = args.windows(2).find(|w| w[0]=="--depth").and_then(|w| w[1].parse().ok()).unwrap_or(0.5f64);
    let height = args.windows(2).find(|w| w[0]=="--height").and_then(|w| w[1].parse().ok()).unwrap_or(3.0f64);
    let fck    = args.windows(2).find(|w| w[0]=="--fck").and_then(|w| w[1].parse().ok()).unwrap_or(25.0f64);
    let fy     = args.windows(2).find(|w| w[0]=="--fy").and_then(|w| w[1].parse().ok()).unwrap_or(415.0f64);
    let dl     = args.windows(2).find(|w| w[0]=="--dl").and_then(|w| w[1].parse().ok()).unwrap_or(3.0f64);
    let ll     = args.windows(2).find(|w| w[0]=="--ll").and_then(|w| w[1].parse().ok()).unwrap_or(2.0f64);
    let wl     = args.windows(2).find(|w| w[0]=="--wind").and_then(|w| w[1].parse().ok()).unwrap_or(1.0f64);
    let zone   = args.windows(2).find(|w| w[0]=="--zone").map(|w| w[1].as_str()).unwrap_or("III");
    let axial  = args.windows(2).find(|w| w[0]=="--axial" ).and_then(|w| w[1].parse().ok()).unwrap_or(500.0f64);
    let sbc    = args.windows(2).find(|w| w[0]=="--sbc").and_then(|w| w[1].parse().ok()).unwrap_or(150.0f64);
    let vb_w   = args.windows(2).find(|w| w[0]=="--vb").and_then(|w| w[1].parse().ok()).unwrap_or(33.0f64);
    let sa_g   = args.windows(2).find(|w| w[0]=="--sa").and_then(|w| w[1].parse().ok()).unwrap_or(2.5f64);
    let r_fac  = args.windows(2).find(|w| w[0]=="--r").and_then(|w| w[1].parse().ok()).unwrap_or(5.0f64);
    let i_fac  = args.windows(2).find(|w| w[0]=="--i").and_then(|w| w[1].parse().ok()).unwrap_or(1.0f64);
    let seisw  = args.windows(2).find(|w| w[0]=="--seismic-weight").and_then(|w| w[1].parse().ok()).unwrap_or(1000.0f64);
    let ly     = args.windows(2).find(|w| w[0]=="--ly").and_then(|w| w[1].parse().ok()).unwrap_or(span * 1.5);
    let profile = args.windows(2).find(|w| w[0]=="--profile").map(|w| w[1].as_str()).unwrap_or("ISMB 200");
    let cf     = args.windows(2).find(|w| w[0]=="--cf").and_then(|w| w[1].parse().ok()).unwrap_or(1.3f64);
    let area_w = args.windows(2).find(|w| w[0]=="--area").and_then(|w| w[1].parse().ok()).unwrap_or(10.0f64);

    match args[1].as_str() {
        "beam"       => cmd_beam(span, udl, pl, cantilever, json),
        "column"     => cmd_column(width, depth, height, fck, fy, axial, json),
        "slab"       => cmd_slab(span, ly, dl, ll, fck, fy, json),
        "foundation" => cmd_foundation(width, depth, axial, sbc, json),
        "wind"       => cmd_wind(vb_w, 1.0, 0.98, 1.0, cf, area_w, json),
        "seismic"    => cmd_seismic(seisw, zone, sa_g, r_fac, i_fac, json),
        "combo"      => cmd_combo(dl, ll, wl, json),
        "section"    => cmd_section(profile, json),
        _ => { eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]); exit(1); }
    }
}
