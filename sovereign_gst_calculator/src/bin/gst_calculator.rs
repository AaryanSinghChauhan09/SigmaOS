// SovereignGSTCalculator CLI
// Command-line interface for GST calculation system

use sovereign_gst_calculator::{GSTRate, GSTCalculator, InvoiceItem, Party, SupplyType};
use std::io::{self, Write};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }
    
    let mut calculator = GSTCalculator::new();
    
    match args[1].as_str() {
        "calculate" => handle_calculate(&calculator, &args),
        "invoice" => handle_invoice(&mut calculator, &args),
        "list" => handle_list(&calculator),
        "itc" => handle_itc(&calculator, &args),
        _ => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    println!("SovereignGSTCalculator CLI");
    println!();
    println!("Usage:");
    println!("  gst_calculator calculate <quantity> <price> <rate> <supply_type>");
    println!("  gst_calculator invoice <invoice_no> <supplier_name> <supplier_gstin> <recipient_name> <recipient_gstin> <supply_type> <item_desc> <item_qty> <item_price> <item_rate>");
    println!("  gst_calculator list");
    println!("  gst_calculator itc <purchase_qty> <purchase_price> <purchase_rate> <sale_qty> <sale_price> <sale_rate>");
    println!();
    println!("Example:");
    println!("  gst_calculator calculate 10 1000 18 intra");
    println!("  gst_calculator invoice INV001 SupplierInc 27ABCDE1234F1Z5 CustomerInc 27ABCDE5678F1Z5 intra Product 5 1000 18");
    println!("  gst_calculator itc 10 100 18 5 100 18");
}

fn handle_calculate(calculator: &GSTCalculator, args: &[String]) {
    if args.len() < 6 {
        eprintln!("Error: Insufficient arguments for calculate command");
        print_usage();
        std::process::exit(1);
    }
    
    let quantity: u64 = args[2].parse().expect("Invalid quantity");
    let price: u64 = args[3].parse().expect("Invalid price");
    let rate: u64 = args[4].parse().expect("Invalid rate");
    let supply_type_str = &args[5];
    
    let gst_rate = GSTRate::from_percentage(rate);
    let supply_type = match supply_type_str.as_str() {
        "intra" => SupplyType::IntraState,
        "inter" => SupplyType::InterState,
        _ => {
            eprintln!("Error: Invalid supply type. Use 'intra' or 'inter'");
            std::process::exit(1);
        }
    };
    
    let item = InvoiceItem::new(
        "Item".to_string(),
        "0000".to_string(),
        quantity,
        price,
        gst_rate,
    );
    
    let breakdown = calculator.calculate_tax(&[item], supply_type);
    
    println!("{}", breakdown);
}

fn handle_invoice(calculator: &mut GSTCalculator, args: &[String]) {
    if args.len() < 12 {
        eprintln!("Error: Insufficient arguments for invoice command");
        print_usage();
        std::process::exit(1);
    }
    
    let invoice_no = args[2].clone();
    let supplier_name = args[3].clone();
    let supplier_gstin = args[4].clone();
    let recipient_name = args[5].clone();
    let recipient_gstin = args[6].clone();
    let supply_type_str = &args[7];
    let item_desc = args[8].clone();
    let item_qty: u64 = args[9].parse().expect("Invalid quantity");
    let item_price: u64 = args[10].parse().expect("Invalid price");
    let item_rate: u64 = args[11].parse().expect("Invalid rate");
    
    let supplier = Party::new(supplier_name, supplier_gstin, "Supplier Address".to_string());
    let recipient = Party::new(recipient_name, recipient_gstin, "Recipient Address".to_string());
    
    let supply_type = match supply_type_str.as_str() {
        "intra" => SupplyType::IntraState,
        "inter" => SupplyType::InterState,
        _ => {
            eprintln!("Error: Invalid supply type. Use 'intra' or 'inter'");
            std::process::exit(1);
        }
    };
    
    let item = InvoiceItem::new(
        item_desc,
        "0000".to_string(),
        item_qty,
        item_price,
        GSTRate::from_percentage(item_rate),
    );
    
    let invoice_id = calculator.create_invoice(
        invoice_no,
        supplier,
        recipient,
        vec![item],
        supply_type,
    );
    
    println!("Invoice created successfully!");
    println!("Invoice ID: {}", invoice_id);
    println!();
    
    if let Some(invoice) = calculator.get_invoice(&invoice_id) {
        println!("{}", invoice);
    }
}

fn handle_list(calculator: &GSTCalculator) {
    let invoices = calculator.list_invoices();
    
    if invoices.is_empty() {
        println!("No invoices found.");
        return;
    }
    
    println!("Invoices ({}):", invoices.len());
    println!();
    
    for invoice in invoices {
        println!("No: {}", invoice.invoice_no);
        println!("ID: {}", invoice.get_invoice_id());
        println!("Supplier: {}", invoice.supplier.name);
        println!("Recipient: {}", invoice.recipient.name);
        println!("Total: ₹{}", invoice.total_amount());
        println!();
    }
}

fn handle_itc(calculator: &GSTCalculator, args: &[String]) {
    if args.len() < 8 {
        eprintln!("Error: Insufficient arguments for itc command");
        print_usage();
        std::process::exit(1);
    }
    
    let purchase_qty: u64 = args[2].parse().expect("Invalid purchase quantity");
    let purchase_price: u64 = args[3].parse().expect("Invalid purchase price");
    let purchase_rate: u64 = args[4].parse().expect("Invalid purchase rate");
    let sale_qty: u64 = args[5].parse().expect("Invalid sale quantity");
    let sale_price: u64 = args[6].parse().expect("Invalid sale price");
    let sale_rate: u64 = args[7].parse().expect("Invalid sale rate");
    
    let purchases = vec![
        InvoiceItem::new(
            "Purchase".to_string(),
            "0000".to_string(),
            purchase_qty,
            purchase_price,
            GSTRate::from_percentage(purchase_rate),
        ),
    ];
    
    let sales = vec![
        InvoiceItem::new(
            "Sale".to_string(),
            "0000".to_string(),
            sale_qty,
            sale_price,
            GSTRate::from_percentage(sale_rate),
        ),
    ];
    
    let report = calculator.reconcile_itc(&purchases, &sales);
    
    println!("{}", report);
}
