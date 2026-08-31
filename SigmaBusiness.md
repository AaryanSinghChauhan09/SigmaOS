# SigmaBusiness

**SigmaBusiness** is the SigmaOS alternative to Odoo and other ERP suites.

## Features

### SigmaCRM (Customer Relationship Management)
Customer relationship management:
- Contact and lead management
- Sales pipeline tracking
- Opportunity management
- Customer communication history
- Email integration with SigmaMail
- Calendar integration with SigmaCalendar
- Sales forecasting and reporting
- AI-powered lead scoring
- Customer segmentation
- Mobile CRM app

### SigmaAccounting (Accounting & Finance)
Ledger, invoicing, and expenses:
- Double-entry bookkeeping
- General ledger and chart of accounts
- Accounts payable and receivable
- Invoice generation and tracking
- Expense management
- Financial reporting (balance sheet, P&L, cash flow)
- Tax compliance (GST, TDS, VAT)
- Bank reconciliation
- Multi-currency support
- AI-powered anomaly detection for fraud

### SigmaInventory (Inventory Management)
Comprehensive inventory tracking:
- Product catalog management
- Stock level monitoring
- Purchase order management
- Sales order processing
- Warehouse management
- Barcode/QR code scanning
- Serial number tracking
- Lot and batch tracking
- Inventory valuation (FIFO, LIFO, weighted average)
- AI-powered demand forecasting

### SigmaHR (Human Resources)
Human resources management:
- Employee records and profiles
- Attendance and time tracking
- Payroll processing
- Leave management
- Performance reviews
- Recruitment and onboarding
- Training and development
- Employee self-service portal
- Compliance and reporting
- AI-powered talent matching

### SigmaProjects (Project Management)
Project management and tracking:
- Project planning and scheduling
- Task management and assignment
- Gantt charts and timelines
- Resource allocation
- Time tracking and billing
- Milestone tracking
- Collaboration tools
- Project reporting and analytics
- AI-powered risk assessment

## Architecture

```
SigmaBusiness Suite
   ├─ SigmaCRM (CRM engine)
   │   ├─ Contact database
   │   ├─ Sales pipeline tracker
   │   ├─ Lead scoring engine (AI)
   │   └─ Communication logger
   ├─ SigmaAccounting (accounting engine)
   │   ├─ General ledger
   │   ├─ Invoice generator
   │   ├─ Tax calculator
   │   └─ Financial reporter
   ├─ SigmaInventory (inventory engine)
   │   ├─ Product catalog
   │   ├─ Stock monitor
   │   ├─ Purchase order processor
   │   └─ Demand forecaster (AI)
   ├─ SigmaHR (HR engine)
   │   ├─ Employee database
   │   ├─ Payroll processor
   │   ├─ Attendance tracker
   │   └─ Recruitment manager
   └─ SigmaProjects (project engine)
       ├─ Project scheduler
       ├─ Task manager
       ├─ Resource allocator
       └─ Risk assessor (AI)
```

## API Interface

```c
// SigmaCRM API
int sigma_crm_add_contact(const contact_t *contact);
int sigma_crm_add_lead(const lead_t *lead);
int sigma_crm_update_pipeline(const char *lead_id, const char *stage);
int sigma_crm_get_communications(const char *contact_id, communication_t *comms, size_t *count);
int sigma_crm_score_lead(const char *lead_id, float *score);

// SigmaAccounting API
int sigma_accounting_create_invoice(const invoice_t *invoice);
int sigma_accounting_record_payment(const char *invoice_id, payment_t *payment);
int sigma_accounting_generate_report(report_type_t type, time_t start, time_t end, report_t *report);
int sigma_accounting_reconcile_bank(const char *account_id, time_t start, time_t end);
int sigma_accounting_calculate_tax(const char *invoice_id, tax_t *tax);

// SigmaInventory API
int sigma_inventory_add_product(const product_t *product);
int sigma_inventory_update_stock(const char *product_id, int quantity);
int sigma_inventory_create_purchase_order(const purchase_order_t *po);
int sigma_inventory_forecast_demand(const char *product_id, forecast_t *forecast);
int sigma_inventory_valuation(valuation_method_t method, valuation_t *val);

// SigmaHR API
int sigma_hr_add_employee(const employee_t *employee);
int sigma_hr_process_payroll(time_t period, payroll_t *payroll);
int sigma_hr_record_attendance(const char *employee_id, attendance_t *att);
int sigma_hr_manage_leave(const char *employee_id, leave_t *leave);
int sigma_hr_performance_review(const char *employee_id, review_t *review);

// SigmaProjects API
int sigma_projects_create_project(const project_t *project);
int sigma_projects_add_task(const char *project_id, task_t *task);
int sigma_projects_allocate_resource(const char *project_id, const char *resource_id);
int sigma_projects_track_time(const char *task_id, time_t hours);
int sigma_projects_generate_report(const char *project_id, project_report_t *report);
```

## Integration

- **SigmaOffice Integration**: Document generation with SigmaWriter, spreadsheets with SigmaSheet
- **SigmaCloud Integration**: Email via SigmaMail, calendar via SigmaCalendar, file storage via SigmaDrive
- **SigmaAI Integration**: AI-powered lead scoring, demand forecasting, risk assessment, fraud detection
- **SigmaFS Integration**: Document storage with SovereignFS snapshots
- **Zenith Desktop Integration**: Native Zenith UI with dashboards and reports

## Compliance

- **GST Compliance**: Indian GST filing and reporting (GSTR-1, GSTR-3B)
- **TDS Compliance**: Tax deduction at source calculation and reporting
- **Accounting Standards**: Compliance with IFRS and local accounting standards
- **Labor Laws**: Compliance with local labor regulations
- **Data Privacy**: GDPR and local data protection law compliance

## Performance Characteristics

| Module | Concurrent Users | Data Size | Real-time Updates |
|---|---|---|---|
| SigmaCRM | 1000+ | 10GB+ | ✅ Yes |
| SigmaAccounting | 500+ | 100GB+ | ✅ Yes |
| SigmaInventory | 1000+ | 50GB+ | ✅ Yes |
| SigmaHR | 500+ | 20GB+ | ✅ Yes |
| SigmaProjects | 500+ | 30GB+ | ✅ Yes |

## Roadmap

- [x] Architecture design and component specification
- [ ] SigmaCRM contact and pipeline implementation
- [ ] SigmaAccounting ledger and invoicing implementation
- [ ] SigmaInventory stock and PO implementation
- [ ] SigmaHR payroll and attendance implementation
- [ ] SigmaProjects task and resource implementation
- [ ] GST/TDS compliance modules
- [ ] AI-powered features (lead scoring, demand forecasting, risk assessment)
- [ ] Mobile apps (SigmaOS Mobile, iOS, Android)
- [ ] Web interface (SigmaOS Cloud)
- [ ] Multi-tenant SaaS version

## Related Modules

- [`suites/SigmaOffice`](../SigmaOffice/README.md) — Office suite integration
- [`suites/SigmaCloud`](../SigmaCloud/README.md) — Cloud suite integration
- [`modules/core/fs`](../../modules/core/fs/README.md) — Filesystem integration
- [`security/pqc/README.md`](../../security/pqc/README.md) — Cryptographic security
