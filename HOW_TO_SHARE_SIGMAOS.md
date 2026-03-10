# 🌍 How to Package, Distribute & Share SigmaOS

Because SigmaOS acts as a complete sovereign ecosystem overlay, sharing it requires caution. By default, SigmaOS stores incredibly powerful ephemeral session states, hardware decryption flags, and zero-trust cookies on your machine.

Follow these strict guidelines before pushing this OS to GitHub, sending it as a ZIP/USB, or compiling it as a standalone executable.

---

## 🚫 1. What NEVER to Share (The Sovereign Warning)
**Never upload/send the Identity Vault or Active Sessions caches to public servers.**
Before packaging, ensure the following simulated directories/data points are **cleared**:
1.  **Any Hardcoded OAuth Tokens**: If you temporarily added keys to test `kernel/identity_vault.py`.
2.  **`SIGMA_VAULT/`** or `.sigma_keys/` directories (if you ever generated an RSA/Hardware key while testing, these must stay with the User).
3.  Because SigmaOS implements the **Identity Vault** Zero-Trust Model, any generated short-lived session token or cookie jar from `kernel/privacy_shield.py` must be wiped out.
   - *Best Practice:* Trigger the **"Revoke All Sessions"** cascade from the Identity UI before packaging.

---

## 📦 2. Standard Packaging: The Sovereign Source Zip
To share the raw, modifiable source code with another developer or engineer:
1.  Create a clean fork of the root folder: `SigmaOS/`
2.  Ensure you have excluded the `__pycache__` folders:
    ```bash
    # (Inside PowerShell/CMD) Delete __pycache__ folders recursively
    FOR /d /r . %d in (__pycache__) DO @IF EXIST "%d" rd /s /q "%d"
    ```
3.  Zip the folder.
4.  The receiver only needs to exact it, make sure they have Python 3.10 installed, and double click `boot_sigma.bat` to launch their own pristine version of the OS Sandbox.

---

## 🚀 3. Compiling to a Standalone Executable (No Python Needed via PyInstaller)
If you want to share SigmaOS with a non-technical user (so they don't even need to install Python), compile it into a single `.exe` file.

**Step 1:** Install PyInstaller inside your environment.
```bash
pip install pyinstaller
```

**Step 2:** Compile the OS Graphical Interface into a unified executable from your terminal. Replace the dark icon parameter with a custom `.ico` if you have one.
```bash
pyinstaller --noconfirm --onedir --windowed --add-data "kernel;kernel/" --add-data "sigma_core;sigma_core/"  "sigma_gui.py"
```

**Step 3:** Check the `dist` folder.
PyInstaller will output a massive `dist/sigma_gui/` folder holding the `.exe` and all its dependencies. **You can directly zip this exact folder** and send it via USB Drive or Cloud Link. When the user opens the zip and clicks `sigma_gui.exe`, the OS overlay will launch flawlessly over Windows.

---

## 📚 4. GitHub / Open Source Distribution
If uploading SigmaOS to GitHub or GitLab:
1. Initialize your repository. `git init`
2. Create a rigorous `.gitignore` file.
    ```text
    # Recommended .gitignore rules for SigmaOS
    __pycache__/
    *.pyc
    *.log
    .sigma_keys/
    sessions/
    dist/
    build/
    *.spec
    ```
3. Highlight to your open-source contributors that SigmaOS incorporates the Zero-Trust Identity Framework—they must fork and create their own sovereign keychains upon initial boot.

---
*Powered by Sovereign Distribution Protocols | The future of offline overlay computing.*
