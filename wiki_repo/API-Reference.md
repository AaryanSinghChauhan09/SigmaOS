# Sovereign SDK API Reference

The Sovereign SDK provides the bridge between your code and the SigmaOS kernel.

## 📁 Core Primitives (`sigma_types.h`)

- `sigma_u32`, `sigma_u64`: Unsigned integers.

- `sigma_status_t`: Status codes (`SIGMA_OK`, `SIGMA_ERROR`).

## 🛠 Logging (`sigma_log.h`)

- `sigma_log_info(fmt, ...)`: Emit informational log.

- `sigma_log_crit(fmt, ...)`: Emit critical fault log.

## 🧬 Industrial Shards (`sigma_sdk.h`)

- `vakil_search(query)`: Search legal database (Indian Law).

- `viz_render_dicom(data, size)`: Render medical imaging.

- `pai_skill(id, params)`: Invoke AI skill.

- `auto_heal(sid, prof)`: Trigger manual healing for a shard.

## 🔌 UI & Graphics (`zenith_compositor.h`)

- `zenith_init(fb_addr)`: Initialize the UI compositor.

- `zenith_apply_theme(theme)`: Update the system theme.

---

### For full function prototypes, see the [include/](include/) directory
 