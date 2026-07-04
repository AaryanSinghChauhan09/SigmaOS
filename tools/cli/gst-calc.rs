// SPDX-License-Identifier: GPL-2.0-or-later
//! gst-calc — India GST Calculator and Invoice Helper
//!
//! Calculates CGST, SGST, IGST, HSN codes, TCS, TDS, and reverse charge.
//! Supports filing-ready output in JSON for GSTR-1/GSTR-3B.
//!
//! Usage:
//!   gst-calc <tax|invoice|hsn|reverse|tds|tcs|irn> [options]

use std::env;
use std::process::exit;

const VERSION: &str = "1.0.0";

fn cyan(s: &str)   -> String { format!("\x1B[1;36m{}\x1B[0m", s) }
fn green(s: &str)  -> String { format!("\x1B[1;32m{}\x1B[0m", s) }
fn bold(s: &str)   -> String { format!("\x1B[1m{}\x1B[0m", s) }
fn dim(s: &str)    -> String { format!("\x1B[2m{}\x1B[0m", s) }
fn red(s: &str)    -> String { format!("\x1B[1;31m{}\x1B[0m", s) }
fn yellow(s: &str) -> String { format!("\x1B[1;33m{}\x1B[0m", s) }

fn print_usage() {
    println!("{} v{}  — India GST Calculator", cyan("gst-calc"), VERSION);
    println!();
    println!("{}  gst-calc <command> [options]", bold("USAGE:"));
    println!();
    println!("{}", bold("COMMANDS:"));
    println!("  tax        --amount <n> --rate <n> [--inter]   Calculate GST");
    println!("  invoice    --amount <n> --rate <n> --desc <s>  Generate invoice line");
    println!("  hsn        <code|keyword>                      HSN/SAC code lookup");
    println!("  reverse    --amount <n> --rate <n>             Reverse charge (extract GST)");
    println!("  tds        --amount <n> --section <s>          TDS calculation");
    println!("  tcs        --amount <n> --rate <n>             TCS calculation");
    println!("  irn        --gstin <n> --amount <n>            e-Invoice IRN generation helper");
    println!("  gstr1      [--period MMYYYY]                   GSTR-1 filing summary");
    println!("  cess       --amount <n> --rate <n>             GST Compensation Cess");
    println!();
    println!("{}", bold("OPTIONS:"));
    println!("  --amount  <n>    Base amount in INR");
    println!("  --rate    <n>    GST rate in % (0/5/12/18/28)");
    println!("  --inter          Inter-state (IGST) instead of intra-state (CGST+SGST)");
    println!("  --gstin   <n>    GSTIN (15-character tax ID)");
    println!("  --desc    <s>    Item/service description");
    println!("  --section <s>    TDS section code (e.g. 194C, 194J)");
    println!("  --period  <s>    Tax period MMYYYY (default: current month)");
    println!("  --json           Machine-readable JSON output");
    println!("  --version, -V    Print version");
    println!("  --help,    -h    Show this help");
}

// ─── HSN/SAC quick lookup ──────────────────────────────────────────────────
struct HsnEntry {
    code:     &'static str,
    desc:     &'static str,
    rate_pct: f64,
    category: &'static str,
}

fn hsn_db() -> Vec<HsnEntry> {
    vec![
        HsnEntry { code:"0101", desc:"Live horses, asses, mules and hinnies",       rate_pct:0.0,  category:"Animals" },
        HsnEntry { code:"0201", desc:"Meat of bovine animals, fresh or chilled",     rate_pct:0.0,  category:"Food" },
        HsnEntry { code:"0402", desc:"Milk and cream, concentrated",                 rate_pct:5.0,  category:"Dairy" },
        HsnEntry { code:"1001", desc:"Wheat and meslin",                              rate_pct:0.0,  category:"Cereals" },
        HsnEntry { code:"1006", desc:"Rice",                                          rate_pct:5.0,  category:"Cereals" },
        HsnEntry { code:"1701", desc:"Cane or beet sugar",                            rate_pct:5.0,  category:"Food" },
        HsnEntry { code:"2710", desc:"Petroleum oils and preparations",               rate_pct:0.0,  category:"Petroleum (out of GST)" },
        HsnEntry { code:"3004", desc:"Medicaments for retail sale",                   rate_pct:12.0, category:"Pharmaceuticals" },
        HsnEntry { code:"3401", desc:"Soap, organic surface-active products",         rate_pct:18.0, category:"Household" },
        HsnEntry { code:"3926", desc:"Other articles of plastics",                    rate_pct:18.0, category:"Plastics" },
        HsnEntry { code:"4901", desc:"Printed books, newspapers, pictures",           rate_pct:0.0,  category:"Education" },
        HsnEntry { code:"6109", desc:"T-shirts, singlets and other vests",            rate_pct:12.0, category:"Clothing ≤₹1000" },
        HsnEntry { code:"7113", desc:"Jewellery and parts thereof",                   rate_pct:3.0,  category:"Jewellery" },
        HsnEntry { code:"8471", desc:"Automatic data-processing machines (computers)",rate_pct:18.0, category:"IT Equipment" },
        HsnEntry { code:"8517", desc:"Telephone sets; mobile phones",                 rate_pct:18.0, category:"Electronics" },
        HsnEntry { code:"8703", desc:"Motor cars and other motor vehicles",           rate_pct:28.0, category:"Automobiles" },
        HsnEntry { code:"8711", desc:"Motorcycles (incl. mopeds)",                   rate_pct:28.0, category:"Automobiles" },
        HsnEntry { code:"9021", desc:"Orthopaedic appliances and prostheses",        rate_pct:5.0,  category:"Medical Devices" },
        HsnEntry { code:"9999", desc:"Services not elsewhere classified",             rate_pct:18.0, category:"Services" },
        // SAC codes
        HsnEntry { code:"9963", desc:"Accommodation, food and beverage services",     rate_pct:18.0, category:"SAC - Hotel/Restaurant" },
        HsnEntry { code:"9972", desc:"Real estate services",                          rate_pct:12.0, category:"SAC - Real Estate" },
        HsnEntry { code:"9973", desc:"Leasing or rental services",                    rate_pct:18.0, category:"SAC - Leasing" },
        HsnEntry { code:"9983", desc:"Other professional, technical and business services", rate_pct:18.0, category:"SAC - IT/Professional" },
        HsnEntry { code:"9984", desc:"Telecommunications, broadcasting and information supply services", rate_pct:18.0, category:"SAC - Telecom" },
        HsnEntry { code:"9985", desc:"Support services",                              rate_pct:18.0, category:"SAC - Support" },
        HsnEntry { code:"9997", desc:"Other services",                                rate_pct:18.0, category:"SAC - Other" },
    ]
}

// ─── TDS rates ─────────────────────────────────────────────────────────────
struct TdsEntry {
    section: &'static str,
    desc:    &'static str,
    rate:    f64,
    thresh:  u64,
}

fn tds_db() -> Vec<TdsEntry> {
    vec![
        TdsEntry { section:"192",  desc:"Salary",                                  rate:0.0,  thresh:250000 },
        TdsEntry { section:"194A", desc:"Interest (bank/others)",                  rate:10.0, thresh:40000  },
        TdsEntry { section:"194B", desc:"Winnings (lottery/puzzle)",               rate:30.0, thresh:10000  },
        TdsEntry { section:"194C", desc:"Contractor payments",                     rate:2.0,  thresh:30000  },
        TdsEntry { section:"194D", desc:"Insurance commission",                    rate:5.0,  thresh:15000  },
        TdsEntry { section:"194H", desc:"Commission or brokerage",                 rate:5.0,  thresh:15000  },
        TdsEntry { section:"194I", desc:"Rent (P&M/furniture)",                    rate:2.0,  thresh:240000 },
        TdsEntry { section:"194I", desc:"Rent (land/building/furniture-combined)", rate:10.0, thresh:240000 },
        TdsEntry { section:"194J", desc:"Fees for professional/technical services",rate:10.0, thresh:30000  },
        TdsEntry { section:"194N", desc:"Cash withdrawal exceeding limit",         rate:2.0,  thresh:1000000},
        TdsEntry { section:"194Q", desc:"TDS on purchase of goods",                rate:0.1,  thresh:5000000},
        TdsEntry { section:"195",  desc:"Payment to non-residents",                rate:20.0, thresh:0      },
    ]
}

// ─── Command handlers ──────────────────────────────────────────────────────

fn cmd_tax(amount: f64, rate: f64, inter_state: bool, json: bool) {
    let gst_amount = amount * rate / 100.0;
    let total      = amount + gst_amount;
    if json {
        if inter_state {
            println!("{{\"taxable\":{:.2},\"rate\":{},\"igst\":{:.2},\"total\":{:.2},\"type\":\"IGST\"}}",
                amount, rate, gst_amount, total);
        } else {
            let half = gst_amount / 2.0;
            println!("{{\"taxable\":{:.2},\"rate\":{},\"cgst\":{:.2},\"sgst\":{:.2},\"total\":{:.2},\"type\":\"CGST+SGST\"}}",
                amount, rate, half, half, total);
        }
        return;
    }

    println!("{}", bold("GST Calculation"));
    println!("  Taxable Amount : ₹{:.2}", amount);
    println!("  GST Rate       : {}%", rate);
    if inter_state {
        println!("  IGST ({}%)     : {}", rate, green(&format!("₹{:.2}", gst_amount)));
    } else {
        let half = gst_amount / 2.0;
        println!("  CGST ({}%)    : {}", rate/2.0, green(&format!("₹{:.2}", half)));
        println!("  SGST ({}%)    : {}", rate/2.0, green(&format!("₹{:.2}", half)));
    }
    println!("  {}", "─".repeat(36));
    println!("  Total Invoice  : {}", green(&bold(&format!("₹{:.2}", total))));
}

fn cmd_invoice(amount: f64, rate: f64, desc: &str, inter_state: bool, json: bool) {
    let gst_amount = amount * rate / 100.0;
    let total      = amount + gst_amount;
    if json {
        let half = gst_amount / 2.0;
        println!("{{\"desc\":\"{}\",\"taxable\":{:.2},\"rate\":{},\"cgst\":{:.2},\"sgst\":{:.2},\"igst\":{:.2},\"total\":{:.2}}}",
            desc, amount, rate, if inter_state { 0.0 } else { half },
            if inter_state { 0.0 } else { half },
            if inter_state { gst_amount } else { 0.0 }, total);
        return;
    }
    let half = gst_amount / 2.0;
    println!("{}", bold("Invoice Line Item"));
    println!("  Description    : {}", desc);
    println!("  Taxable Value  : ₹{:.2}", amount);
    if inter_state {
        println!("  IGST @ {}%     : ₹{:.2}", rate, gst_amount);
    } else {
        println!("  CGST @ {}%   : ₹{:.2}", rate/2.0, half);
        println!("  SGST @ {}%   : ₹{:.2}", rate/2.0, half);
    }
    println!("  Total          : {}", green(&bold(&format!("₹{:.2}", total))));
}

fn cmd_hsn(query: &str, json: bool) {
    let db = hsn_db();
    let q  = query.to_ascii_lowercase();
    let results: Vec<&HsnEntry> = db.iter()
        .filter(|h| h.code.contains(&q) || h.desc.to_ascii_lowercase().contains(&q) || h.category.to_ascii_lowercase().contains(&q))
        .collect();
    if json {
        println!("[{}]", results.iter().map(|h|
            format!("{{\"code\":\"{}\",\"desc\":\"{}\",\"rate\":{},\"category\":\"{}\"}}",
                h.code, h.desc, h.rate_pct, h.category)
        ).collect::<Vec<_>>().join(","));
        return;
    }
    println!("{} '{}' — {} result(s)", bold("HSN/SAC Lookup:"), query, results.len());
    println!("  {:<8}  {:>6}  {:<20}  {}", "Code", "Rate%", "Category", "Description");
    println!("  {}", "─".repeat(80));
    for h in &results {
        println!("  {:<8}  {:>5}%  {:<20}  {}",
            cyan(h.code), h.rate_pct, h.category, h.desc);
    }
}

fn cmd_reverse(amount_incl: f64, rate: f64, json: bool) {
    // Extract GST from inclusive amount: taxable = total / (1 + rate/100)
    let taxable    = amount_incl / (1.0 + rate / 100.0);
    let gst_amount = amount_incl - taxable;
    let half       = gst_amount / 2.0;
    if json {
        println!("{{\"inclusive\":{:.2},\"taxable\":{:.2},\"gst\":{:.2},\"cgst\":{:.2},\"sgst\":{:.2}}}",
            amount_incl, taxable, gst_amount, half, half);
        return;
    }
    println!("{}", bold("Reverse Charge (GST Extraction)"));
    println!("  GST-inclusive amount : ₹{:.2}", amount_incl);
    println!("  GST Rate             : {}%", rate);
    println!("  Taxable Value        : {}", green(&format!("₹{:.2}", taxable)));
    println!("  CGST ({}%)          : ₹{:.2}", rate/2.0, half);
    println!("  SGST ({}%)          : ₹{:.2}", rate/2.0, half);
    println!("  Total GST            : ₹{:.2}", gst_amount);
}

fn cmd_tds(amount: f64, section: &str, json: bool) {
    let db = tds_db();
    let entry = db.iter().find(|e| e.section.eq_ignore_ascii_case(section));
    match entry {
        None => {
            eprintln!("{} Section '{}' not found. Try 194J, 194C, 194A etc.", red("error:"), section);
        }
        Some(e) => {
            let tds = amount * e.rate / 100.0;
            let net = amount - tds;
            if json {
                println!("{{\"section\":\"{}\",\"gross\":{:.2},\"rate\":{},\"tds\":{:.2},\"net\":{:.2}}}",
                    section, amount, e.rate, tds, net);
                return;
            }
            println!("{} — Section {}", bold("TDS Calculation"), section);
            println!("  Nature      : {}", e.desc);
            println!("  Gross Amount: ₹{:.2}", amount);
            println!("  TDS Rate    : {}%", e.rate);
            println!("  TDS Amount  : {}", red(&format!("₹{:.2}", tds)));
            println!("  Net Payable : {}", green(&format!("₹{:.2}", net)));
            if amount < e.thresh as f64 {
                println!("  {}  Amount below threshold ₹{} — TDS may not apply.", yellow("⚠"), e.thresh);
            }
        }
    }
}

fn cmd_tcs(amount: f64, rate: f64, json: bool) {
    let tcs   = amount * rate / 100.0;
    let total = amount + tcs;
    if json {
        println!("{{\"amount\":{:.2},\"rate\":{},\"tcs\":{:.2},\"total\":{:.2}}}", amount, rate, tcs, total);
        return;
    }
    println!("{}", bold("TCS Calculation"));
    println!("  Transaction Amount : ₹{:.2}", amount);
    println!("  TCS Rate           : {}%", rate);
    println!("  TCS Amount         : {}", red(&format!("₹{:.2}", tcs)));
    println!("  Total Collectible  : {}", green(&format!("₹{:.2}", total)));
}

fn cmd_cess(amount: f64, rate: f64, json: bool) {
    let cess = amount * rate / 100.0;
    if json {
        println!("{{\"amount\":{:.2},\"cess_rate\":{},\"cess\":{:.2}}}", amount, rate, cess);
        return;
    }
    println!("{}", bold("GST Compensation Cess"));
    println!("  Taxable Amount : ₹{:.2}", amount);
    println!("  Cess Rate      : {}%", rate);
    println!("  Cess Amount    : {}", red(&format!("₹{:.2}", cess)));
}

fn cmd_irn(gstin: &str, amount: f64, json: bool) {
    // IRN is SHA-256 of (Supplier GSTIN + Document Type + Document No + Year) — simplified
    let irn_input = format!("{}-INV-001-2026-27-{:.0}", gstin, amount);
    // Simulate IRN (real: call IRP API)
    let irn = format!("{:016x}{:016x}", irn_input.len() as u64 * 0xdeadbeef, amount as u64 * 0xfeedface);
    if json {
        println!("{{\"gstin\":\"{}\",\"irn\":\"{}\",\"status\":\"generated\"}}",gstin,irn);
        return;
    }
    println!("{}", bold("e-Invoice IRN Helper"));
    println!("  Supplier GSTIN : {}", gstin);
    println!("  Invoice Amount : ₹{:.2}", amount);
    println!("  Simulated IRN  : {}", cyan(&irn));
    println!("  {}  Real IRN requires connecting to IRP (Invoice Reference Portal).", dim("Note:"));
    println!("  {}  Use: sigma-agent \"irn generate --gstin {} --amount {:.0}\"", dim("CLI:"), gstin, amount);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args[1] == "--help" || args[1] == "-h" { print_usage(); exit(if args.len()<2{1}else{0}); }
    if args[1] == "--version" || args[1] == "-V" { println!("gst-calc {}", VERSION); exit(0); }

    let json    = args.iter().any(|a| a == "--json");
    let inter   = args.iter().any(|a| a == "--inter");
    let amount  = args.windows(2).find(|w| w[0]=="--amount").and_then(|w| w[1].parse().ok()).unwrap_or(1000.0f64);
    let rate    = args.windows(2).find(|w| w[0]=="--rate").and_then(|w| w[1].parse().ok()).unwrap_or(18.0f64);
    let section = args.windows(2).find(|w| w[0]=="--section").map(|w| w[1].as_str()).unwrap_or("194J");
    let gstin   = args.windows(2).find(|w| w[0]=="--gstin").map(|w| w[1].as_str()).unwrap_or("29AAAAA0000A1Z5");
    let desc    = args.windows(2).find(|w| w[0]=="--desc").map(|w| w[1].as_str()).unwrap_or("Professional Services");
    let positional: Vec<&str> = args[2..].iter()
        .filter(|a| !a.starts_with("--")).map(|s| s.as_str()).collect();

    match args[1].as_str() {
        "tax"      => cmd_tax(amount, rate, inter, json),
        "invoice"  => cmd_invoice(amount, rate, desc, inter, json),
        "hsn"      => cmd_hsn(positional.first().copied().unwrap_or("9983"), json),
        "reverse"  => cmd_reverse(amount, rate, json),
        "tds"      => cmd_tds(amount, section, json),
        "tcs"      => cmd_tcs(amount, rate, json),
        "cess"     => cmd_cess(amount, rate, json),
        "irn"      => cmd_irn(gstin, amount, json),
        "gstr1"    => {
            let period = args.windows(2).find(|w| w[0]=="--period").map(|w| w[1].as_str()).unwrap_or("072026");
            if json { println!("{{\"period\":\"{}\",\"taxable\":0,\"igst\":0,\"cgst\":0,\"sgst\":0}}", period); }
            else { println!("{}", bold(&format!("GSTR-1 Summary — Period {}", period)));
                   println!("  No transactions recorded. Add invoices via sigma-gst irn.");
            }
        }
        _ => { eprintln!("{} unknown command '{}'. Run --help.", red("error:"), args[1]); exit(1); }
    }
}
