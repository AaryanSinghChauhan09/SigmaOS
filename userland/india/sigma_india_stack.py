#!/usr/bin/env python3
# SPDX-License-Identifier: MIT
# Copyright (c) 2024-2026 SigmaOS Project
#
# userland/india/sigma_india_stack.py — India Stack API Client
#
# Implements real API clients for:
#   - ABDM FHIR (Ayushman Bharat Digital Mission — health records)
#   - GST IRN (Invoice Reference Number via NIC portal)
#   - UPI Autopay mandate creation
#   - DigiLocker document fetch
#   - Bhashini NMT translation API
#   - NSDL/PAN verification
#
# Phase H implementation — requires Phase G network stack (TCP/HTTPS).
# Uses urllib.request (no external deps) for HTTP calls.

import hashlib
import json
import os
import time
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass, asdict
from typing import Optional

# ── Configuration ──────────────────────────────────────────────────────────
ABDM_BASE    = os.environ.get("ABDM_BASE",    "https://dev.abdm.gov.in/gateway")
GST_BASE     = os.environ.get("GST_BASE",     "https://einvoice1-uat.nic.in/ei/api")
UPI_BASE     = os.environ.get("UPI_BASE",     "https://api.npci.org.in/upi/autopay")
DIGILOCKER   = os.environ.get("DIGILOCKER",   "https://digilocker.meripehchaan.gov.in")
BHASHINI_BASE= os.environ.get("BHASHINI_BASE","https://dhruva-api.bhashini.gov.in/services/inference")
NSDL_BASE    = os.environ.get("NSDL_BASE",    "https://tin.tin.nsdl.com/panstatus/rest")

CLIENT_ID    = os.environ.get("SIGMA_ABDM_CLIENT_ID",    "sigma-os-dev")
CLIENT_SEC   = os.environ.get("SIGMA_ABDM_CLIENT_SECRET","changeme")
GST_USERNAME = os.environ.get("SIGMA_GST_USERNAME", "testgstin")
GST_APP_KEY  = os.environ.get("SIGMA_GST_APP_KEY",  "testappkey")

# ── HTTP helpers ──────────────────────────────────────────────────────────

def _post(url: str, body: dict, headers: dict = None, timeout: int = 10) -> dict:
    data = json.dumps(body).encode()
    h = {"Content-Type": "application/json", "Accept": "application/json"}
    if headers: h.update(headers)
    req = urllib.request.Request(url, data=data, headers=h, method="POST")
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return json.loads(r.read().decode())
    except urllib.error.HTTPError as e:
        return {"error": e.code, "message": e.read().decode()[:200]}
    except Exception as e:
        return {"error": "network", "message": str(e)}

def _get(url: str, headers: dict = None, timeout: int = 10) -> dict:
    h = {"Accept": "application/json"}
    if headers: h.update(headers)
    req = urllib.request.Request(url, headers=h)
    try:
        with urllib.request.urlopen(req, timeout=timeout) as r:
            return json.loads(r.read().decode())
    except urllib.error.HTTPError as e:
        return {"error": e.code, "message": e.read().decode()[:200]}
    except Exception as e:
        return {"error": "network", "message": str(e)}

# ── 1. ABDM — Ayushman Bharat Digital Mission ──────────────────────────────

class AbdmClient:
    """Ayushman Bharat Digital Mission — FHIR-based health record API."""

    def __init__(self):
        self._token: Optional[str] = None
        self._token_expiry: float = 0

    def _ensure_token(self) -> bool:
        if self._token and time.time() < self._token_expiry:
            return True
        resp = _post(f"{ABDM_BASE}/v0.5/sessions", {
            "clientId": CLIENT_ID,
            "clientSecret": CLIENT_SEC,
        })
        if "accessToken" in resp:
            self._token = resp["accessToken"]
            self._token_expiry = time.time() + resp.get("expiresIn", 3600) - 60
            return True
        return False

    def _auth_headers(self) -> dict:
        return {
            "Authorization": f"Bearer {self._token}",
            "X-CM-ID": "sbx",
        }

    def generate_health_id(self, aadhaar_number: str, mobile: str) -> dict:
        """Create an ABHA (Ayushman Bharat Health Account) ID."""
        if not self._ensure_token():
            return {"error": "auth_failed"}
        return _post(f"{ABDM_BASE}/v2/registration/aadhaar/generateOtp",
            {"aadhaar": aadhaar_number, "mobile": mobile},
            self._auth_headers())

    def fetch_health_records(self, abha_id: str, from_date: str, to_date: str) -> dict:
        """Fetch FHIR health records for an ABHA ID."""
        if not self._ensure_token():
            return {"error": "auth_failed"}
        return _post(f"{ABDM_BASE}/v0.5/health-information/cm/request",
            {
                "requestId": hashlib.md5(abha_id.encode()).hexdigest(),
                "timestamp": _iso_ts(),
                "query": {
                    "healthInformationTypes": ["OPConsultation", "DiagnosticReport", "Prescription"],
                    "dateRange": {"from": from_date, "to": to_date},
                    "patientId": abha_id,
                }
            },
            self._auth_headers())

    def link_health_facility(self, abha_id: str, hip_id: str) -> dict:
        """Link a healthcare facility (HIP) to an ABHA account."""
        if not self._ensure_token():
            return {"error": "auth_failed"}
        return _post(f"{ABDM_BASE}/v0.5/links/link/add-contexts",
            {"requestId": hashlib.md5(f"{abha_id}{hip_id}".encode()).hexdigest(),
             "timestamp": _iso_ts(),
             "link": {"accessToken": "link_token", "patient": {"id": abha_id,
                      "referenceNumber": hip_id, "careContexts": []}}},
            self._auth_headers())

    def verify_otp(self, txn_id: str, otp: str) -> dict:
        if not self._ensure_token():
            return {"error": "auth_failed"}
        return _post(f"{ABDM_BASE}/v2/registration/aadhaar/verifyOTP",
            {"txnId": txn_id, "otp": otp},
            self._auth_headers())

# ── 2. GST — e-Invoice IRN API ─────────────────────────────────────────────

class GstIrnClient:
    """GST e-Invoice API — generates Invoice Reference Numbers via NIC portal."""

    def __init__(self):
        self._auth_token: Optional[str] = None
        self._sek: Optional[str] = None   # Session Encryption Key

    def authenticate(self, gstin: str) -> bool:
        """Authenticate with GST e-Invoice API."""
        resp = _post(f"{GST_BASE}/auth",
            {"Username": GST_USERNAME, "ForceRefreshAccessToken": "true",
             "Gstin": gstin, "OtpRequest": "true"})
        if resp.get("Status") == 1:
            self._auth_token = resp.get("AuthToken")
            self._sek        = resp.get("Sek")
            return True
        return False

    def generate_irn(self, invoice: dict) -> dict:
        """Generate an IRN for an invoice. invoice = B2B invoice dict."""
        if not self._auth_token:
            return {"error": "not_authenticated"}
        # The real API requires AES encryption of the payload using SEK
        # Here we format the request structure correctly
        payload = {
            "Version": "1.1",
            "TranDtls": {
                "TaxSch": "GST",
                "SupTyp": invoice.get("supply_type", "B2B"),
                "RegRev": "N",
            },
            "DocDtls": {
                "Typ": invoice.get("doc_type", "INV"),
                "No": invoice.get("invoice_no"),
                "Dt": invoice.get("invoice_date"),
            },
            "SellerDtls": invoice.get("seller", {}),
            "BuyerDtls":  invoice.get("buyer",  {}),
            "ItemList":   invoice.get("items",   []),
            "ValDtls": {
                "AssVal": invoice.get("assessable_value", 0),
                "CgstVal": invoice.get("cgst", 0),
                "SgstVal": invoice.get("sgst", 0),
                "IgstVal": invoice.get("igst", 0),
                "TotInvVal": invoice.get("total_value", 0),
            },
        }
        return _post(f"{GST_BASE}/invoice",
            {"Data": json.dumps(payload)},
            {"authtoken": self._auth_token, "gstin": invoice.get("seller_gstin", "")})

    def get_irn_details(self, irn: str) -> dict:
        """Fetch details of an existing IRN."""
        return _get(f"{GST_BASE}/invoice/irn/{irn}",
            {"authtoken": self._auth_token or ""})

    def cancel_irn(self, irn: str, reason: int, remarks: str) -> dict:
        """Cancel an IRN (within 24 hours of generation)."""
        return _post(f"{GST_BASE}/invoice/cancel",
            {"Irn": irn, "CnlRsn": str(reason), "CnlRem": remarks},
            {"authtoken": self._auth_token or ""})

# ── 3. UPI Autopay ────────────────────────────────────────────────────────

class UpiAutopayClient:
    """NPCI UPI Autopay — recurring payment mandate management."""

    def create_mandate(
        self, payer_vpa: str, payee_vpa: str,
        amount_paise: int, frequency: str,
        start_date: str, end_date: str,
        purpose: str = "Subscription",
    ) -> dict:
        """Create a recurring UPI Autopay mandate."""
        mandate_id = f"SIGMA{int(time.time()*1000)}"
        return _post(f"{UPI_BASE}/create",
            {
                "mandateId": mandate_id,
                "payerVpa":  payer_vpa,
                "payeeVpa":  payee_vpa,
                "amount":    amount_paise,
                "currency":  "INR",
                "frequency": frequency,    # DAILY, WEEKLY, MONTHLY, ASPRESENTED
                "startDate": start_date,
                "endDate":   end_date,
                "purpose":   purpose,
                "txnInitType": "CREATE",
            })

    def revoke_mandate(self, mandate_id: str, payer_vpa: str) -> dict:
        return _post(f"{UPI_BASE}/revoke",
            {"mandateId": mandate_id, "payerVpa": payer_vpa})

    def mandate_status(self, mandate_id: str) -> dict:
        return _get(f"{UPI_BASE}/status/{mandate_id}")

    def initiate_debit(self, mandate_id: str, amount_paise: int, txn_note: str) -> dict:
        """Trigger a debit against an existing mandate."""
        return _post(f"{UPI_BASE}/debit",
            {"mandateId": mandate_id, "amount": amount_paise,
             "txnNote": txn_note, "txnId": f"STXN{int(time.time()*1000)}"})

# ── 4. DigiLocker ─────────────────────────────────────────────────────────

class DigiLockerClient:
    """DigiLocker — fetch Aadhaar, PAN, driving licence, degree certificates."""

    def __init__(self, access_token: str = ""):
        self._token = access_token

    def fetch_document(self, doc_type: str, doc_id: str) -> dict:
        """Fetch a document. doc_type: ADHAR, PANCR, DRVLC, etc."""
        return _get(f"{DIGILOCKER}/1/xml/pullfile",
            {"Authorization": f"Bearer {self._token}",
             "doctype": doc_type, "docid": doc_id})

    def list_documents(self) -> dict:
        """List all documents in a user's DigiLocker."""
        return _get(f"{DIGILOCKER}/1/xml/files",
            {"Authorization": f"Bearer {self._token}"})

    def issue_certificate(self, department: str, cert_type: str, data: dict) -> dict:
        """Trigger certificate issuance from a government department."""
        return _post(f"{DIGILOCKER}/1/xml/issuedoc",
            {"dept": department, "certType": cert_type, "data": data},
            {"Authorization": f"Bearer {self._token}"})

# ── 5. Bhashini — Indian Language Translation ─────────────────────────────

class BhashiniClient:
    """Bhashini ULCA/Dhruva API — NMT translation, ASR, TTS for Indian languages."""

    LANG_CODES = {
        "hindi": "hi", "tamil": "ta", "telugu": "te", "kannada": "kn",
        "malayalam": "ml", "bengali": "bn", "marathi": "mr", "gujarati": "gu",
        "punjabi": "pa", "odia": "or", "assamese": "as", "urdu": "ur",
        "english": "en",
    }

    def __init__(self, api_key: str = ""):
        self._api_key = api_key or os.environ.get("SIGMA_BHASHINI_KEY", "")

    def translate(self, text: str, src_lang: str, tgt_lang: str) -> str:
        """Translate text from src_lang to tgt_lang."""
        src = self.LANG_CODES.get(src_lang, src_lang)
        tgt = self.LANG_CODES.get(tgt_lang, tgt_lang)
        resp = _post(f"{BHASHINI_BASE}/nmt/inference",
            {
                "pipelineTasks": [{
                    "taskType": "translation",
                    "config": {"language": {"sourceLanguage": src, "targetLanguage": tgt}},
                }],
                "inputData": {"input": [{"source": text}]},
            },
            {"Authorization": self._api_key})
        try:
            return resp["pipelineResponse"][0]["output"][0]["target"]
        except (KeyError, IndexError):
            return f"[translation unavailable: {resp.get('error', 'unknown error')}]"

    def asr_transcribe(self, audio_base64: str, lang: str) -> str:
        """Transcribe speech to text (ASR)."""
        lang_code = self.LANG_CODES.get(lang, lang)
        resp = _post(f"{BHASHINI_BASE}/asr/inference",
            {
                "pipelineTasks": [{
                    "taskType": "asr",
                    "config": {"language": {"sourceLanguage": lang_code}},
                }],
                "inputData": {"audio": [{"audioContent": audio_base64}]},
            },
            {"Authorization": self._api_key})
        try:
            return resp["pipelineResponse"][0]["output"][0]["source"]
        except (KeyError, IndexError):
            return ""

    def tts_synthesize(self, text: str, lang: str, gender: str = "female") -> bytes:
        """Synthesize speech from text (TTS). Returns WAV bytes."""
        lang_code = self.LANG_CODES.get(lang, lang)
        resp = _post(f"{BHASHINI_BASE}/tts/inference",
            {
                "pipelineTasks": [{
                    "taskType": "tts",
                    "config": {"language": {"sourceLanguage": lang_code},
                               "gender": gender},
                }],
                "inputData": {"input": [{"source": text}]},
            },
            {"Authorization": self._api_key})
        try:
            import base64
            audio_b64 = resp["pipelineResponse"][0]["audio"][0]["audioContent"]
            return base64.b64decode(audio_b64)
        except (KeyError, IndexError, Exception):
            return b""

# ── 6. NSDL PAN verification ──────────────────────────────────────────────

class NsdlPanClient:
    """NSDL PAN card status verification."""

    def verify_pan(self, pan: str) -> dict:
        """Verify a PAN card number's validity and status."""
        pan = pan.upper().strip()
        # Basic format check: AAAAA9999A
        import re
        if not re.match(r'^[A-Z]{5}[0-9]{4}[A-Z]$', pan):
            return {"valid": False, "error": "invalid_format", "pan": pan}
        return _post(f"{NSDL_BASE}/json",
            {"getpanstatus": {"pan_number_1": pan, "pan_application_type": "pan"}})

# ── Helper ─────────────────────────────────────────────────────────────────
def _iso_ts() -> str:
    import datetime
    return datetime.datetime.utcnow().strftime("%Y-%m-%dT%H:%M:%S.000Z")

# ── CLI demo ──────────────────────────────────────────────────────────────
def main():
    import argparse
    parser = argparse.ArgumentParser(prog="sigma-india-stack", description="India Stack API Client")
    sub = parser.add_subparsers(dest="cmd", required=True)

    sub.add_parser("abdm-auth",      help="Test ABDM authentication")
    sub.add_parser("gst-auth",       help="Test GST e-Invoice authentication")
    p_tr = sub.add_parser("translate", help="Translate text via Bhashini")
    p_tr.add_argument("text"); p_tr.add_argument("--from", default="english", dest="src")
    p_tr.add_argument("--to",   default="hindi",   dest="tgt")
    p_pan = sub.add_parser("pan",    help="Verify a PAN number")
    p_pan.add_argument("pan_number")
    sub.add_parser("upi-demo",       help="Demo UPI Autopay mandate creation")

    args = parser.parse_args()

    if args.cmd == "abdm-auth":
        client = AbdmClient()
        ok = client._ensure_token()
        print(f"ABDM auth: {'✓ success' if ok else '✗ failed (check env vars SIGMA_ABDM_CLIENT_ID/SECRET)'}")

    elif args.cmd == "gst-auth":
        client = GstIrnClient()
        ok = client.authenticate(os.environ.get("SIGMA_GSTIN", "29AAABB1111D1Z5"))
        print(f"GST auth: {'✓ success' if ok else '✗ failed (check env vars)'}")

    elif args.cmd == "translate":
        client = BhashiniClient()
        result = client.translate(args.text, args.src, args.tgt)
        print(f"  {args.src} → {args.tgt}: {result}")

    elif args.cmd == "pan":
        client = NsdlPanClient()
        result = client.verify_pan(args.pan_number)
        print(json.dumps(result, indent=2))

    elif args.cmd == "upi-demo":
        client = UpiAutopayClient()
        result = client.create_mandate(
            payer_vpa="user@oksbi", payee_vpa="merchant@hdfc",
            amount_paise=9900, frequency="MONTHLY",
            start_date="2026-08-01", end_date="2027-07-31",
            purpose="SigmaOS Subscription",
        )
        print(json.dumps(result, indent=2))

if __name__ == "__main__":
    main()
