import os, re

base_dir = os.path.abspath(os.path.join(os.path.dirname(__file__), '..', 'web_ui'))
app_js_path = os.path.join(base_dir, 'scripts', 'app.js')
index_html_path = os.path.join(base_dir, 'index.html')
modules_dir = os.path.join(base_dir, 'scripts', 'modules')
os.makedirs(modules_dir, exist_ok=True)

with open(app_js_path, 'r', encoding='utf-8') as f:
    text = f.read()

text = re.sub(
    r"method:\s*'POST',(\s*)body:\s*JSON\.stringify",
    r"method: 'POST',\g<1>headers: { 'Content-Type': 'application/json' },\g<1>body: JSON.stringify",
    text
)

config_code = text.split('document.addEventListener')[0]
rest = text.split('document.addEventListener')[1]
rest = rest[rest.find('{')+1 : rest.rfind('}')] 

parts = re.split(r'/\* ={62}\s*\*\s*(.*?)\s*\*\s*={62} \*/', rest)

sections = {}
current_name = 'globals'
sections[current_name] = parts[0]

for i in range(1, len(parts), 2):
    name = parts[i]
    content = parts[i+1]
    sections[name] = content

with open(os.path.join(modules_dir, '01_config.js'), 'w', encoding='utf-8') as f:
    f.write(config_code)

for k, v in sections.items():
    safe_name = re.sub(r'[^a-zA-Z0-9]', '_', k).lower()
    with open(os.path.join(modules_dir, f'02_{safe_name}.js'), 'w', encoding='utf-8') as f:
        f.write('document.addEventListener("DOMContentLoaded", () => {\n' + v.strip() + '\n});')

with open(index_html_path, 'r', encoding='utf-8') as f:
    html = f.read()

html = html.replace('<script src="scripts/app.js"></script>', '''
    <script src="scripts/modules/01_config.js"></script>
    <script src="scripts/modules/02_globals.js"></script>
    <script src="scripts/modules/02_paradigm_selection__boot_menu_.js"></script>
    <script src="scripts/modules/02_gui_mode__zenith_dashboard.js"></script>
    <script src="scripts/modules/02_browser_use__dom_sweep_protocol.js"></script>
    <script src="scripts/modules/02_cli_mode__sovereign_shell.js"></script>
''')

with open(index_html_path, 'w', encoding='utf-8') as f:
    f.write(html)

print('Modularisation complete')
