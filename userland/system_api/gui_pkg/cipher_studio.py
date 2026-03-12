import tkinter as tk
from tkinter import ttk
import os
import webbrowser
import tempfile
from .base_page import SigmaPage
from .styles import PAL, FONT_SMALL, FONT_MED

class CipherStudioPage(SigmaPage):
    def __init__(self, parent, controller):
        super().__init__(parent, controller)
        self.build()

    def build(self):
        self.controller._build_page_header(self, "ZERO-TRUST CIPHER STUDIO", "Browser-based Local Encryption & Hashing")
        
        main_panel = tk.Frame(self, bg=PAL["bg"])
        main_panel.pack(fill="both", expand=True, padx=20, pady=10)
        
        card = self.controller._card(main_panel, "Cryptography Engine Core")
        card.master.pack(pady=50)
        
        tk.Label(card, text="The Cipher Studio executes all cryptographic operations locally within the browser. No data ever leaves the device.",
                 font=FONT_MED, bg=PAL["card"], fg=PAL["dim"], wraplength=400).pack(pady=20, padx=20)
        
        def _launch():
            self._generate_and_launch_html()
            self.controller._notify("Cipher Studio", "Browser encryption suite online.", "OK")

        ttk.Button(card, text="🔐 Launch Cipher Studio in Browser", command=_launch, style="Teal.TButton").pack(pady=20)

    def _generate_and_launch_html(self):
        html_content = """<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>SigmaOS Zero-Trust Cipher Studio</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; background-color: #0A0A12; color: #F2F2F7; margin: 0; padding: 20px; }
        .container { max-width: 900px; margin: 0 auto; background-color: #11111E; padding: 20px; border-radius: 10px; box-shadow: 0 4px 15px rgba(0,0,0,0.5); }
        h1 { color: #5AC8FA; font-size: 24px; text-align: center; }
        h2 { color: #5856D6; font-size: 18px; border-bottom: 1px solid #38383A; padding-bottom: 10px; }
        .grid-2 { display: grid; grid-template-columns: 1fr 1fr; gap: 20px; }
        .card { background-color: #1C1C1E; padding: 15px; border-radius: 8px; border: 1px solid #38383A; }
        input, textarea, select { width: 100%; box-sizing: border-box; padding: 10px; margin-top: 5px; margin-bottom: 15px; background-color: #252529; color: white; border: 1px solid #5856D6; border-radius: 4px; font-family: monospace; }
        .btn { background-color: #5856D6; color: white; padding: 10px; width: 100%; border: none; border-radius: 4px; cursor: pointer; font-weight: bold; font-size: 14px; }
        .btn:hover { background-color: #AF52DE; }
        .output { background-color: #0A0A12; padding: 15px; border-radius: 4px; min-height: 100px; border: 1px solid #5AC8FA; overflow-y: auto; white-space: pre-wrap; font-family: 'Consolas', monospace; color: #5AC8FA; word-break: break-all; }
        .status { font-size: 12px; color: #34C759; margin-top: -10px; margin-bottom: 15px; }
    </style>
    <!-- We use the native crypto.subtle API for 100% standard, no 3rd-party library crypto -->
</head>
<body>
    <div class="container">
        <h1>🔐 SigmaOS Zero-Trust Cipher Studio</h1>
        <div class="status">● All logic operates client-side via WebCrypto API. Data is never transmitted.</div>
        
        <div class="grid-2">
            <!-- Encryption Panel -->
            <div class="card">
                <h2>AES-256-GCM Encryption</h2>
                <label>Secret Secret/Password</label>
                <input type="password" id="enc-pass" placeholder="Enter highly secure phrase...">
                <label>Plaintext Data</label>
                <textarea id="enc-text" rows="4">My highly sensitive data...</textarea>
                <button class="btn" onclick="encryptData()">🔒 Encrypt Data</button>
            </div>
            
            <!-- Decryption Panel -->
            <div class="card">
                <h2>AES-256-GCM Decryption</h2>
                <label>Secret Secret/Password</label>
                <input type="password" id="dec-pass" placeholder="Enter phrase to decrypt...">
                <label>Ciphertext (Hex + IV)</label>
                <textarea id="dec-text" rows="4" placeholder="Paste IV::Ciphertext here..."></textarea>
                <button class="btn" onclick="decryptData()" style="background-color: #34C759; color: black;">🔓 Decrypt Data</button>
            </div>
        </div>

        <div class="card" style="margin-top: 20px;">
            <h2>SHA-256 File / String Hashing</h2>
            <div class="grid-2">
                <div>
                    <label>String to Hash</label>
                    <input type="text" id="hash-text" placeholder="String...">
                    <button class="btn" onclick="hashString()" style="background-color: #FF9F0A;">#️⃣ Generate Hash</button>
                </div>
            </div>
        </div>
        
        <div class="card" style="margin-top: 20px;">
            <h2>Security Log & Output</h2>
            <div class="output" id="out-console">Cipher Engine Initialized. WebCrypto API available.</div>
        </div>
    </div>

    <script>
        const out = document.getElementById('out-console');

        // Helper: Convert string to ArrayBuffer
        function str2ab(str) {
            return new TextEncoder().encode(str);
        }
        // Helper: Convert ArrayBuffer to Hex String
        function buf2hex(buffer) {
            return Array.prototype.map.call(new Uint8Array(buffer), x => ('00' + x.toString(16)).slice(-2)).join('');
        }
        // Helper: Convert Hex String to Uint8Array
        function hex2buf(hexString) {
            let result = [];
            for (let i=0; i < hexString.length; i+=2) {
                result.push(parseInt(hexString.substr(i, 2), 16));
            }
            return new Uint8Array(result);
        }

        // Generate Key Material from Password (PBKDF2)
        async function getKeyMaterial(password) {
            return window.crypto.subtle.importKey(
                "raw", str2ab(password), {name: "PBKDF2"}, false, ["deriveBits", "deriveKey"]
            );
        }

        async function getKey(keyMaterial, salt) {
            return window.crypto.subtle.deriveKey(
                { name: "PBKDF2", salt: salt, iterations: 100000, hash: "SHA-256" },
                keyMaterial,
                { name: "AES-GCM", length: 256 },
                true, ["encrypt", "decrypt"]
            );
        }

        async function encryptData() {
            try {
                const pass = document.getElementById('enc-pass').value;
                const text = document.getElementById('enc-text').value;

                if(!pass || !text) throw new Error("Missing password or plaintext.");

                const salt = window.crypto.getRandomValues(new Uint8Array(16));
                const iv = window.crypto.getRandomValues(new Uint8Array(12));
                
                const keyMaterial = await getKeyMaterial(pass);
                const key = await getKey(keyMaterial, salt);
                
                const encoded = str2ab(text);
                const ciphertextBuffer = await window.crypto.subtle.encrypt(
                    { name: "AES-GCM", iv: iv }, key, encoded
                );
                
                const cipherHex = buf2hex(ciphertextBuffer);
                const ivHex = buf2hex(iv);
                const saltHex = buf2hex(salt);

                const finalBundle = saltHex + "::" + ivHex + "::" + cipherHex;
                
                document.getElementById('dec-text').value = finalBundle;
                out.innerText = `[ENCRYPT OK] AES-256-GCM computed in JS engine.\\nBUNDLE: ${finalBundle}\\n\\n` + out.innerText;
            } catch(e) {
                out.innerText = `[ENCRYPT ERR] ${e.message}\\n\\n` + out.innerText;
            }
        }

        async function decryptData() {
            try {
                const pass = document.getElementById('dec-pass').value;
                const bundle = document.getElementById('dec-text').value;
                
                if(!pass || !bundle) throw new Error("Missing password or ciphertext bundle.");

                const parts = bundle.split("::");
                if(parts.length !== 3) throw new Error("Invalid bundle format. Expected SALT::IV::CIPHER");
                
                const salt = hex2buf(parts[0]);
                const iv = hex2buf(parts[1]);
                const ciphertext = hex2buf(parts[2]);

                const keyMaterial = await getKeyMaterial(pass);
                const key = await getKey(keyMaterial, salt);
                
                const decryptedBuffer = await window.crypto.subtle.decrypt(
                    { name: "AES-GCM", iv: iv }, key, ciphertext
                );

                const decryptedText = new TextDecoder().decode(decryptedBuffer);
                out.innerText = `[DECRYPT OK] Data restored.\\nPLAINTEXT: ${decryptedText}\\n\\n` + out.innerText;
            } catch(e) {
                 out.innerText = `[DECRYPT ERR] Authentication failed or corrupted data: ${e.message}\\n\\n` + out.innerText;
            }
        }

        async function hashString() {
            try {
                const text = document.getElementById('hash-text').value;
                const msgUint8 = str2ab(text);
                const hashBuffer = await crypto.subtle.digest('SHA-256', msgUint8);
                const hashHex = buf2hex(hashBuffer);
                
                out.innerText = `[HASH OK] SHA-256 computed.\\nINPUT: ${text}\\nHASH: ${hashHex}\\n\\n` + out.innerText;
            } catch (e) {
                out.innerText = `[HASH ERR] ${e.message}\\n\\n` + out.innerText;
            }
        }
    </script>
</body>
</html>
        """
        path = os.path.join(tempfile.gettempdir(), "sigma_cipher_studio.html")
        with open(path, "w", encoding="utf-8") as f:
            f.write(html_content)
        webbrowser.open("file://" + os.path.realpath(path))
