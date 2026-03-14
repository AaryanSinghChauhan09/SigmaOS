"""
SigmaOS Statutory Grand Library (v1.0 Apex Data)
================================================
USP: Exhaustive Dictionary of Indian Legal Acts and Forms.
Modularized from LegalFormEngine to separate data from logic.
"""

GRAND_LIBRARY = {
    # --- BNSS 2023 & BNS (Criminal) ---
    "BNSS_FIR_173": {
        "act": "BNSS 2023", "section": "173", "title": "Criminal Complaint / FIR",
        "fields": [
            {"id": "station", "label": "Police Station", "type": "text"},
            {"id": "offense", "label": "Nature of Offense", "type": "text"},
            {"id": "accused", "label": "Accused Details", "type": "textarea"},
            {"id": "incident", "label": "Incident Narrative", "type": "textarea"}
        ]
    },
    "BNSS_ANTICIPATORY_BAIL": {
        "act": "BNSS 2023", "section": "482", "title": "Anticipatory Bail Application",
        "fields": [
            {"id": "court", "label": "Sessions Court / High Court", "type": "text"},
            {"id": "apprehension", "label": "Reason for Apprehension", "type": "textarea"}
        ]
    },
    # --- CPC 1908 (Civil) ---
    "CPC_PLAINT_CIVIL": {
        "act": "CPC 1908", "section": "Order VII", "title": "Civil Plaint (Suit Recovery)",
        "fields": [
            {"id": "court", "label": "Civil Judge (Sr/Jr Div)", "type": "text"},
            {"id": "valuation", "label": "Suit Valuation (INR)", "type": "text"},
            {"id": "cause", "label": "Specific Cause of Action", "type": "textarea"}
        ]
    },
    # --- NI Act (Financial) ---
    "NI_SEC138_NOTICE": {
        "act": "Negotiable Instruments Act", "section": "138", "title": "Legal Notice for Cheque Bounce",
        "fields": [
            {"id": "cheque_no", "label": "Cheque Number", "type": "text"},
            {"id": "bank", "label": "Drawee Bank", "type": "text"},
            {"id": "amount", "label": "Cheque Amount", "type": "text"},
            {"id": "demand_days", "label": "Demand Period (15 days)", "type": "text"}
        ]
    },
    # --- Consumer Protection Act ---
    "CONSUMER_COMPLAINT": {
        "act": "Consumer Protection Act 2019", "section": "Sec 35", "title": "Consumer Commission Complaint",
        "fields": [
            {"id": "commission", "label": "District/State Commission", "type": "text"},
            {"id": "deficiency", "label": "Deficiency in Service", "type": "textarea"},
            {"id": "compensation", "label": "Compensation Claimed", "type": "text"}
        ]
    },
    # --- RTI Act (Transparency) ---
    "RTI_APPLICATION": {
        "act": "Right to Information Act 2005", "section": "6(1)", "title": "RTI Information Request",
        "fields": [
            {"id": "pio", "label": "Public Information Officer", "type": "text"},
            {"id": "dept", "label": "Public Authority", "type": "text"},
            {"id": "info", "label": "Information Required", "type": "textarea"}
        ]
    },
    # --- Motor Vehicles Act ---
    "MVA_CLAIM_PETITION": {
        "act": "Motor Vehicles Act 1988", "section": "166", "title": "MACT Claim Petition",
        "fields": [
            {"id": "mact", "label": "MACT Tribunal Name", "type": "text"},
            {"id": "vehicle_no", "label": "Vehicle Number", "type": "text"},
            {"id": "injury", "label": "Nature of Injury", "type": "textarea"}
        ]
    },
    # --- Family Courts Act ---
    "FAMILY_MUTUAL_DIVORCE": {
        "act": "Family Courts Act", "section": "13B", "title": "Mutual Divorce Petition",
        "fields": [
            {"id": "court", "label": "Family Court at...", "type": "text"},
            {"id": "settlement", "label": "Terms of Settlement", "type": "textarea"}
        ]
    },
    # --- BSA 2023 (Digital Evidence) ---
    "BSA_DIGITAL_63": {
        "act": "BSA 2023", "section": "63", "title": "Certificate for Digital Evidence",
        "fields": [
            {"id": "dev_id", "label": "Device Details", "type": "text"},
            {"id": "hash", "label": "SHA-256 Hash", "type": "text"},
            {"id": "certifier", "label": "Certified By", "type": "text"}
        ]
    }
}
