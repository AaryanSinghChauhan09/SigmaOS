// SovereignGSTCalculator - GST Calculation System
// Implements GST Act / Income Tax Act compliance
// No external dependencies - implements from first principles

use std::fmt;

/// GST rate slabs per GST Act
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GSTRate {
    Nil,      // 0%
    Exempted, // 0%
    Rate5,    // 5%
    Rate12,   // 12%
    Rate18,   // 18%
    Rate28,   // 28%
}

impl GSTRate {
    pub fn from_percentage(percent: u64) -> Self {
        match percent {
            0 => GSTRate::Nil,
            5 => GSTRate::Rate5,
            12 => GSTRate::Rate12,
            18 => GSTRate::Rate18,
            28 => GSTRate::Rate28,
            _ => panic!("Invalid GST rate: {}", percent),
        }
    }
    
    pub fn as_percentage(&self) -> u64 {
        match self {
            GSTRate::Nil => 0,
            GSTRate::Exempted => 0,
            GSTRate::Rate5 => 5,
            GSTRate::Rate12 => 12,
            GSTRate::Rate18 => 18,
            GSTRate::Rate28 => 28,
        }
    }
    
    pub fn as_decimal(&self) -> f64 {
        self.as_percentage() as f64 / 100.0
    }
}

/// Supply type (intra-state vs inter-state)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupplyType {
    IntraState,  // CGST + SGST
    InterState,  // IGST
}

/// Tax breakdown
#[derive(Debug, Clone)]
pub struct TaxBreakdown {
    pub cgst: u64,
    pub sgst: u64,
    pub igst: u64,
    pub cess: u64,
    pub total_tax: u64,
    pub total_amount: u64,
}

impl TaxBreakdown {
    pub fn new(cgst: u64, sgst: u64, igst: u64, cess: u64, total_amount: u64) -> Self {
        let total_tax = cgst + sgst + igst + cess;
        TaxBreakdown {
            cgst,
            sgst,
            igst,
            cess,
            total_tax,
            total_amount,
        }
    }
}

impl fmt::Display for TaxBreakdown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Tax Breakdown:\n\
             CGST: ₹{}\n\
             SGST: ₹{}\n\
             IGST: ₹{}\n\
             CESS: ₹{}\n\
             Total Tax: ₹{}\n\
             Total Amount: ₹{}",
            self.cgst, self.sgst, self.igst, self.cess, self.total_tax, self.total_amount
        )
    }
}

/// Invoice item
#[derive(Debug, Clone)]
pub struct InvoiceItem {
    pub description: String,
    pub hsn_code: String,
    pub quantity: u64,
    pub unit_price: u64,
    pub gst_rate: GSTRate,
}

impl InvoiceItem {
    pub fn new(description: String, hsn_code: String, quantity: u64, unit_price: u64, gst_rate: GSTRate) -> Self {
        InvoiceItem {
            description,
            hsn_code,
            quantity,
            unit_price,
            gst_rate,
        }
    }
    
    pub fn base_amount(&self) -> u64 {
        self.quantity * self.unit_price
    }
    
    pub fn tax_amount(&self) -> u64 {
        let base = self.base_amount();
        let rate = self.gst_rate.as_decimal();
        (base as f64 * rate) as u64
    }
    
    pub fn total_amount(&self) -> u64 {
        self.base_amount() + self.tax_amount()
    }
}

/// Party (supplier or recipient)
#[derive(Debug, Clone)]
pub struct Party {
    pub name: String,
    pub gstin: String,
    pub address: String,
}

impl Party {
    pub fn new(name: String, gstin: String, address: String) -> Self {
        Party {
            name,
            gstin,
            address,
        }
    }
}

/// GST Invoice
#[derive(Debug, Clone)]
pub struct GSTInvoice {
    pub invoice_id: [u8; 32],
    pub invoice_no: String,
    pub invoice_date: u64,
    pub supplier: Party,
    pub recipient: Party,
    pub items: Vec<InvoiceItem>,
    pub tax_breakdown: TaxBreakdown,
    pub supply_type: SupplyType,
}

impl GSTInvoice {
    pub fn new(
        invoice_no: String,
        supplier: Party,
        recipient: Party,
        items: Vec<InvoiceItem>,
        supply_type: SupplyType,
    ) -> Self {
        let invoice_id = Self::generate_invoice_id(&invoice_no);
        let invoice_date = Self::current_timestamp();
        let tax_breakdown = Self::calculate_tax_breakdown(&items, supply_type);
        
        GSTInvoice {
            invoice_id,
            invoice_no,
            invoice_date,
            supplier,
            recipient,
            items,
            tax_breakdown,
            supply_type,
        }
    }
    
    fn generate_invoice_id(invoice_no: &str) -> [u8; 32] {
        // Placeholder for actual BLAKE3 hash
        let mut hash = [0u8; 32];
        let bytes = invoice_no.as_bytes();
        for (i, &byte) in bytes.iter().enumerate() {
            hash[i % 32] = hash[i % 32].wrapping_add(byte);
        }
        hash
    }
    
    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    
    fn calculate_tax_breakdown(items: &[InvoiceItem], supply_type: SupplyType) -> TaxBreakdown {
        let total_base: u64 = items.iter().map(|item| item.base_amount()).sum();
        let total_tax: u64 = items.iter().map(|item| item.tax_amount()).sum();
        
        match supply_type {
            SupplyType::IntraState => {
                let cgst = total_tax / 2;
                let sgst = total_tax - cgst;
                TaxBreakdown::new(cgst, sgst, 0, 0, total_base + total_tax)
            }
            SupplyType::InterState => {
                TaxBreakdown::new(0, 0, total_tax, 0, total_base + total_tax)
            }
        }
    }
    
    pub fn get_invoice_id(&self) -> String {
        self.invoice_id.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<Vec<_>>()
            .join("")
    }
    
    pub fn total_amount(&self) -> u64 {
        self.tax_breakdown.total_amount
    }
}

impl fmt::Display for GSTInvoice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "GST Invoice\n\
             Invoice No: {}\n\
             Invoice ID: {}\n\
             Date: {}\n\
             Supplier: {}\n\
             Recipient: {}\n\
             Supply Type: {:?}\n\
             \n\
             Items:\n",
            self.invoice_no,
            self.get_invoice_id(),
            self.invoice_date,
            self.supplier.name,
            self.recipient.name,
            self.supply_type
        )?;
        
        for (i, item) in self.items.iter().enumerate() {
            writeln!(
                f,
                "  {}. {} - {} x ₹{} = ₹{} (GST: {}%)",
                i + 1,
                item.description,
                item.quantity,
                item.unit_price,
                item.base_amount(),
                item.gst_rate.as_percentage()
            )?;
        }
        
        write!(f, "\n{}\n", self.tax_breakdown)
    }
}

/// GST Calculator
pub struct GSTCalculator {
    invoices: Vec<GSTInvoice>,
}

impl GSTCalculator {
    pub fn new() -> Self {
        GSTCalculator {
            invoices: Vec::new(),
        }
    }
    
    /// Calculate tax for items
    pub fn calculate_tax(&self, items: &[InvoiceItem], supply_type: SupplyType) -> TaxBreakdown {
        GSTInvoice::calculate_tax_breakdown(items, supply_type)
    }
    
    /// Create invoice
    pub fn create_invoice(
        &mut self,
        invoice_no: String,
        supplier: Party,
        recipient: Party,
        items: Vec<InvoiceItem>,
        supply_type: SupplyType,
    ) -> String {
        let invoice = GSTInvoice::new(invoice_no, supplier, recipient, items, supply_type);
        let invoice_id = invoice.get_invoice_id();
        
        self.invoices.push(invoice);
        
        invoice_id
    }
    
    /// Get invoice by ID
    pub fn get_invoice(&self, invoice_id: &str) -> Option<&GSTInvoice> {
        self.invoices
            .iter()
            .find(|i| i.get_invoice_id() == invoice_id)
    }
    
    /// List all invoices
    pub fn list_invoices(&self) -> Vec<&GSTInvoice> {
        self.invoices.iter().collect()
    }
    
    /// Calculate ITC (Input Tax Credit) reconciliation
    pub fn reconcile_itc(&self, purchases: &[InvoiceItem], sales: &[InvoiceItem]) -> ITCReport {
        let purchase_tax: u64 = purchases.iter().map(|item| item.tax_amount()).sum();
        let sales_tax: u64 = sales.iter().map(|item| item.tax_amount()).sum();
        let itc_available = purchase_tax.saturating_sub(sales_tax);
        
        ITCReport {
            purchase_tax,
            sales_tax,
            itc_available,
        }
    }
}

/// ITC Report
#[derive(Debug, Clone)]
pub struct ITCReport {
    pub purchase_tax: u64,
    pub sales_tax: u64,
    pub itc_available: u64,
}

impl fmt::Display for ITCReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ITC Reconciliation Report\n\
             Purchase Tax: ₹{}\n\
             Sales Tax: ₹{}\n\
             ITC Available: ₹{}",
            self.purchase_tax, self.sales_tax, self.itc_available
        )
    }
}

impl Default for GSTCalculator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_gst_rate_conversion() {
        assert_eq!(GSTRate::from_percentage(5), GSTRate::Rate5);
        assert_eq!(GSTRate::from_percentage(18), GSTRate::Rate18);
        assert_eq!(GSTRate::Rate18.as_percentage(), 18);
    }
    
    #[test]
    fn test_invoice_item_calculation() {
        let item = InvoiceItem::new(
            "Test Item".to_string(),
            "1234".to_string(),
            10,
            100,
            GSTRate::Rate18,
        );
        
        assert_eq!(item.base_amount(), 1000);
        assert_eq!(item.tax_amount(), 180);
        assert_eq!(item.total_amount(), 1180);
    }
    
    #[test]
    fn test_tax_breakdown_intra_state() {
        let items = vec![
            InvoiceItem::new("Item".to_string(), "1234".to_string(), 10, 100, GSTRate::Rate18),
        ];
        
        let breakdown = GSTInvoice::calculate_tax_breakdown(&items, SupplyType::IntraState);
        
        assert_eq!(breakdown.cgst, 90);
        assert_eq!(breakdown.sgst, 90);
        assert_eq!(breakdown.igst, 0);
        assert_eq!(breakdown.total_tax, 180);
    }
    
    #[test]
    fn test_tax_breakdown_inter_state() {
        let items = vec![
            InvoiceItem::new("Item".to_string(), "1234".to_string(), 10, 100, GSTRate::Rate18),
        ];
        
        let breakdown = GSTInvoice::calculate_tax_breakdown(&items, SupplyType::InterState);
        
        assert_eq!(breakdown.cgst, 0);
        assert_eq!(breakdown.sgst, 0);
        assert_eq!(breakdown.igst, 180);
        assert_eq!(breakdown.total_tax, 180);
    }
    
    #[test]
    fn test_invoice_creation() {
        let supplier = Party::new(
            "Supplier Inc".to_string(),
            "27ABCDE1234F1Z5".to_string(),
            "Supplier Address".to_string(),
        );
        
        let recipient = Party::new(
            "Customer Inc".to_string(),
            "27ABCDE5678F1Z5".to_string(),
            "Customer Address".to_string(),
        );
        
        let items = vec![
            InvoiceItem::new("Product".to_string(), "1234".to_string(), 5, 1000, GSTRate::Rate18),
        ];
        
        let invoice = GSTInvoice::new(
            "INV001".to_string(),
            supplier,
            recipient,
            items,
            SupplyType::IntraState,
        );
        
        assert_eq!(invoice.invoice_no, "INV001");
        assert_eq!(invoice.total_amount(), 5900);
    }
    
    #[test]
    fn test_itc_reconciliation() {
        let calculator = GSTCalculator::new();
        
        let purchases = vec![
            InvoiceItem::new("Purchase".to_string(), "1234".to_string(), 10, 100, GSTRate::Rate18),
        ];
        
        let sales = vec![
            InvoiceItem::new("Sale".to_string(), "1234".to_string(), 5, 100, GSTRate::Rate18),
        ];
        
        let report = calculator.reconcile_itc(&purchases, &sales);
        
        assert_eq!(report.purchase_tax, 180);
        assert_eq!(report.sales_tax, 90);
        assert_eq!(report.itc_available, 90);
    }
}
