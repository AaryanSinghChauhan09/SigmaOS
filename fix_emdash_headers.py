"""Rewrite the 24 files with bad em-dash on line 1 of their docstring."""
import ast, os

ROOT = r'C:\Users\SigmaUser\Downloads\SigmaOS'
BADS = [
    r'sigma_core\system\update_manager\sigmaupdatemanager\check_for_updates.py',
    r'sigma_core\system\update_manager\sigmaupdatemanager\hot_patch_module.py',
    r'sigma_core\system\update_manager\sigmaupdatemanager\rollback_module_hot.py',
    r'sigma_core\system\update_manager\sigmaupdatemanager\set_bandwidth_schedule.py',
    r'sigma_core\system\update_manager\sigmaupdatemanager\simulate_interrupted_update.py',
    r'sigma_core\system\update_manager\_deltashard\verify.py',
    r'sigma_core\ui\fluid_design\fluidtheme\apply_to_widget.py',
    r'sigma_core\ui\fluid_design\fluidtheme\get_font.py',
    r'sigma_core\ui\fluid_design\fluidtheme\set_vibe.py',
    r'sigma_core\ui\ghostchat\sigmaghostchat\send_message.py',
    r'sigma_core\ui\ghostchat\sigmaghostchat\start_service.py',
    r'sigma_core\ui\ghostchat\sigmaghostchat\_handle_peer.py',
    r'sigma_core\ui\ghostchat\sigmaghostchat\_listen_for_peers.py',
    r'sigma_core\ui\ghostchat\sigmaghostchat\_peer_discovery.py',
    r'sigma_core\ui\ghostchat\sigmaghostchat\_shred_volatile_memory.py',
    r'userland\system_api\unified_api\sigmaunifiedapi\add_interceptor.py',
    r'userland\system_api\unified_api\sigmaunifiedapi\register_function.py',
    r'userland\system_api\unified_api\sigmaunifiedapi\_dispatch.py',
    r'userland\system_api\update_manager\sigmaupdatemanager\check_for_updates.py',
    r'userland\system_api\update_manager\sigmaupdatemanager\hot_patch_module.py',
    r'userland\system_api\update_manager\sigmaupdatemanager\rollback_module_hot.py',
    r'userland\system_api\update_manager\sigmaupdatemanager\set_bandwidth_schedule.py',
    r'userland\system_api\update_manager\sigmaupdatemanager\simulate_interrupted_update.py',
    r'userland\system_api\update_manager\_deltashard\verify.py',
]

fixed = 0
for rel in BADS:
    fp = os.path.join(ROOT, rel)
    if not os.path.exists(fp):
        continue

    raw = open(fp, 'rb').read()

    # Find the end of the triple-quote docstring (first occurrence of triple-quote on each side)
    # Strategy: find third newline — line 1 is opener, line 2 is description, line 3 is closer
    lines_raw = raw.split(b'\n')
    body_lines = []
    in_docstring = True
    for i, line in enumerate(lines_raw):
        if in_docstring:
            # once we see the closing """, we exit docstring
            decoded = line.decode('utf-8', errors='replace')
            if i > 0 and decoded.strip() == '"""':
                in_docstring = False
            continue
        body_lines.append(line.decode('utf-8', errors='replace'))

    new_src = '# auto-split module\n' + '\n'.join(body_lines)
    try:
        ast.parse(new_src)
        with open(fp, 'w', encoding='utf-8') as f:
            f.write(new_src)
        fixed += 1
        print(f'  FIXED: {rel}')
    except SyntaxError as e:
        print(f'  FAIL:  {rel}: {e}')

print(f'\nDone: {fixed}/{len(BADS)} files rewritten cleanly.')
