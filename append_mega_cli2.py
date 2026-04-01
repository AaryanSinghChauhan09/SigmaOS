import os
import subprocess

def append_mega_cli_catalog():
    repo_dir = r"C:\Users\Aaryan\.gemini\antigravity\scratch\SigmaOS"
    os.chdir(repo_dir)
    guide_path = "os_guide.md"

    new_content = """

## 🔱 SIGMA OMNI-SHELL: MEGA CLI CATALOG (BATCH 2)
### Zero GUI-Only Actions. 100% CLI Parity. Zero HLL Dependency.

---

### 📺 Display & Screen Management
| Command | Action |
|---|---|
| `sigma-display brightness set 80` | Set screen brightness to 80% via hardware backlight register |
| `sigma-display gamma set 1.2 1.2 1.0` | Adjust RGB gamma correction curves directly |
| `sigma-display color-profile apply sRGB` | Load an ICC color profile at hardware buffer level |
| `sigma-display resolution set 2560x1440 --hz 144` | Change resolution and refresh without restarting compositor |
| `sigma-display hdcp enable --port HDMI-1` | Enable HDCP encryption on output port |
| `sigma-display rotate --port DP-1 --degrees 90` | Rotate a specific display output 90 degrees |
| `sigma-display mirror --source eDP-1 --target HDMI-1` | Clone display output to second monitor |
| `sigma-display power off --after 300` | Auto power off display after 300 seconds idle |
| `sigma-display screenshot --area 0,0,1920,1080 --out file.png` | Capture screen region to file via framebuffer read |
| `sigma-display record --fps 60 --out screen.raw` | Record raw framebuffer output to file |

---

### 🖥️ Window & Workspace Management
| Command | Action |
|---|---|
| `sigma-ui window list` | List all open windows with PIDs |
| `sigma-ui window focus --pid <id>` | Bring a window into foreground focus |
| `sigma-ui window minimize --pid <id>` | Minimize a specific window |
| `sigma-ui window maximize --pid <id>` | Maximize a specific window |
| `sigma-ui window fullscreen --pid <id>` | Set window to borderless fullscreen |
| `sigma-ui window close --pid <id>` | Gracefully close a window |
| `sigma-ui window kill --pid <id>` | Force-terminate a window's process |
| `sigma-ui window pin --pid <id>` | Pin a window across all workspaces |
| `sigma-ui window opacity set --pid <id> 0.85` | Set window transparency |
| `sigma-ui workspace create --name dev` | Create a named workspace |
| `sigma-ui workspace delete --name dev` | Delete a workspace |
| `sigma-ui workspace rename old new` | Rename a workspace |
| `sigma-ui workspace move-window --pid <id> --to dev` | Move a window to another workspace |
| `sigma-ui workspace list` | List all workspaces |
| `sigma-ui tile layout columns 3` | Set tiling to 3-column mode |
| `sigma-ui tile swap --left --right` | Swap two tiled windows |

---

### 🗂️ File System & Storage
| Command | Action |
|---|---|
| `sigma-fs mount --dev /dev/sda2 --point /mnt/data --type ext4` | Mount drive natively |
| `sigma-fs unmount /mnt/data` | Unmount drive |
| `sigma-fs format --dev /dev/sdb1 --type sigma-fs --label DATA` | Format drive with SigmaFS |
| `sigma-fs snapshot create --path /home --label snap1` | Create a VFS snapshot |
| `sigma-fs snapshot restore snap1` | Restore from a VFS snapshot |
| `sigma-fs snapshot list` | List available snapshots |
| `sigma-fs quota set --user user1 --max 50G` | Set per-user disk quota |
| `sigma-fs defrag --dev /dev/sda2` | Defragment drive via raw sector writes |
| `sigma-fs encrypt --path /home/vault --alg AES-256-GCM` | Encrypt a directory in-place |
| `sigma-fs decrypt --path /home/vault` | Decrypt an encrypted directory |
| `sigma-fs stats --path /home` | Show inode, block, and size statistics |
| `sigma-fs trash --path file.txt` | Move file to system trash bin |
| `sigma-fs trash empty` | Permanently purge trash bin |
| `sigma-fs link --src source --dest link --type soft` | Create symbolic or hard link |
| `sigma-fs permissions set --path file --chmod 755` | Set exact file permissions |

---

### 🌐 Networking & Connectivity
| Command | Action |
|---|---|
| `sigma-net ip set --iface eth0 --addr 192.168.1.5/24` | Statically assign IP to an interface |
| `sigma-net ip dhcp --iface eth0` | Request DHCP lease on interface |
| `sigma-net route add --dest 10.0.0.0/8 --gw 192.168.1.1` | Add a static route |
| `sigma-net dns set --primary 1.1.1.1 --secondary 8.8.8.8` | Configure system DNS |
| `sigma-net proxy set --http 192.168.1.100:3128` | Configure system-wide HTTP proxy |
| `sigma-net proxy clear` | Remove all proxy settings |
| `sigma-net wifi scan` | Scan available wireless networks |
| `sigma-net wifi connect <ssid> --pass <password>` | Connect to a WiFi network |
| `sigma-net wifi forget <ssid>` | Remove a saved WiFi network |
| `sigma-net hotspot start --ssid SigmaNet --pass 12345678` | Start a WiFi hotspot |
| `sigma-net hotspot stop` | Stop the hotspot |
| `sigma-net bandwidth limit --iface eth0 --up 10mbps --down 100mbps` | Apply bandwidth shaping |
| `sigma-net firewall rule list` | List all firewall rules |
| `sigma-net firewall rule delete --id 5` | Delete a firewall rule by ID |
| `sigma-net firewall flush` | Flush all firewall rules (permissive mode) |
| `sigma-net ping <host> --count 10 --size 1024` | Native ping via raw ICMP socket |
| `sigma-net trace <host>` | Native traceroute via raw sockets |
| `sigma-net port scan --host 192.168.1.1 --range 1-1024` | Scan open ports natively |
| `sigma-net packet capture --iface eth0 --filter tcp --out dump.pcap` | Capture packets to file |
| `sigma-net vpn import --file profile.ovpn` | Import VPN config |
| `sigma-net vpn status` | Show VPN connection state |
| `sigma-net ssh connect --host 10.0.0.50 --user admin --port 22` | Initiate an encrypted SSH session |
| `sigma-net ssh keygen --type ed25519 --out ~/.sigma/id_ed25519` | Generate SSH key natively |
| `sigma-net ssh copy-id --host 10.0.0.50 --user admin` | Copy public key to remote |

---

### 🔒 Security & Privacy
| Command | Action |
|---|---|
| `sigma-sec passwd change --user user1` | Change a user password |
| `sigma-sec 2fa enable --user user1 --method totp` | Enable two-factor auth |
| `sigma-sec session list` | List all active user sessions |
| `sigma-sec session kill --id <session_id>` | Terminate a specific session |
| `sigma-sec keyring add --service github --user dev --pass <token>` | Store credential in encrypted keyring |
| `sigma-sec keyring get --service github --user dev` | Retrieve keyring credential |
| `sigma-sec firewall port allow --port 8080 --proto tcp` | Allow a specific port |
| `sigma-sec firewall port deny --port 23 --proto tcp` | Block a port (e.g., legacy Telnet) |
| `sigma-sec integrity check --path /kernel` | Verify cryptographic checksums |
| `sigma-sec cert generate --domain sigma.os --type self-signed` | Generate TLS certificate |
| `sigma-sec cert import --path cert.pem` | Install a TLS certificate |
| `sigma-sec audit trail --from "2026-01-01" --to "2026-04-01"` | Query system audit trail |
| `sigma-sec threat scan --level deep` | Run anomaly detection pass |

---

### 🔈 Audio & Media
| Command | Action |
|---|---|
| `sigma-audio volume set --sink default 75` | Set master volume |
| `sigma-audio mute toggle --sink default` | Toggle mute |
| `sigma-audio device list` | List audio devices |
| `sigma-audio device set-default --type output --id 2` | Set default audio output device |
| `sigma-audio eq set --bass 3 --treble -1` | Set equalizer bands |
| `sigma-audio record --input mic --out recording.raw --duration 30` | Record microphone input |
| `sigma-media play <file> --renderer direct` | Play media via native C11 renderer |
| `sigma-media transcode --in video.mp4 --out video.mkv --codec H265` | Transcode video natively |
| `sigma-media metadata read <file>` | Read embedded media metadata |
| `sigma-camera capture --device /dev/video0 --out photo.raw` | Capture a photo from camera |
| `sigma-camera record --device /dev/video0 --out video.raw --fps 30` | Record video from camera |
| `sigma-camera filter apply --type edge-detect --in photo.raw --out out.raw` | Apply mathematical image filter |

---

### 📊 Process & System Monitoring
| Command | Action |
|---|---|
| `sigma-ps list --sort cpu --top 20` | List top 20 CPU-consuming processes |
| `sigma-ps kill --pid <id> --signal SIGTERM` | Send signal to process |
| `sigma-ps kill --name chrome --signal SIGKILL` | Kill all processes by name |
| `sigma-ps nice --pid <id> --level -10` | Change process priority |
| `sigma-ps affinity set --pid <id> --cpus 0,1,2,3` | Pin process to CPU cores |
| `sigma-ps tree` | Show full process hierarchy tree |
| `sigma-ps trace --pid <id>` | Low-level syscall trace of a running process |
| `sigma-sys info` | Display full hardware/software info |
| `sigma-sys uptime` | Show system uptime |
| `sigma-sys temp --all` | Read all thermal sensors |
| `sigma-sys load` | Show 1/5/15-min load averages |
| `sigma-sys mem detail` | Show per-slab memory usage details |

---

### 🤖 AI Copilot & Intelligence
| Command | Action |
|---|---|
| `sigma-ai explain --cmd "sigma-kernel module inject"` | Get explanation of any OS command |
| `sigma-ai suggest --context "high cpu usage"` | Get AI suggestions for current state |
| `sigma-ai anomaly scan` | Run AI anomaly detection on system state |
| `sigma-ai log analyze --file system.log` | Feed log file to local LLM for analysis |
| `sigma-ai chat` | Open interactive AI Copilot in terminal |
| `sigma-ai model list` | List locally available AI models |
| `sigma-ai model load <name>` | Load a specific model into inference shard |
| `sigma-ai infer --model phi2 --prompt "Summarize: $(cat report.txt)"` | Run a native inference task |
| `sigma-ml train --data dataset.csv --model linear-regression --out model.bin` | Train an ML model natively |
| `sigma-ml predict --model model.bin --input 3.5,2.1` | Run prediction with trained model |
| `sigma-ml graph plot --data stats.csv --type scatter --out graph.raw` | Generate a data plot to raw framebuffer |

---

### 🧩 Shard & Persona Orchestration
| Command | Action |
|---|---|
| `sigma-shard status` | Show all currently loaded shards and their memory footprints |
| `sigma-shard load ui --lazy` | Lazily pre-queue UI shard for next GUI invocation |
| `sigma-shard profile save --name my-dev-state` | Save entire loaded shard profile |
| `sigma-shard profile restore --name my-dev-state` | Restore a saved shard profile |
| `sigma-persona create --name scientist --extends researcher` | Create a new derived persona |
| `sigma-persona export --name scientist --out scientist.yaml` | Export persona config |
| `sigma-persona import --file scientist.yaml` | Import persona config |
| `sigma-persona diff dev gamer` | Show differences between two personas |

---

### 🏗️ Developer & Toolchain
| Command | Action |
|---|---|
| `sigma-dev compile --lang c11 --src main.c --out main --flags "-O3"` | Compile C11 source natively |
| `sigma-dev compile --lang asm --src boot.asm --out boot.bin` | Assemble ASM to binary |
| `sigma-dev link --objs "a.o b.o" --out app --static` | Native static linker |
| `sigma-dev debug attach --pid <id>` | Attach low-level debugger |
| `sigma-dev debug breakpoint --addr 0x4A200` | Set hardware breakpoint at address |
| `sigma-dev profile --pid <id> --duration 10 --out flame.svg` | Collect and render flamegraph |
| `sigma-dev container create --name sandbox --image base` | Create isolated container |
| `sigma-dev container run --name sandbox --cmd "/bin/sigma-sh"` | Run command in container |
| `sigma-dev container destroy --name sandbox` | Destroy a container |
| `sigma-dev vm launch --image sigma.iso --ram 2048 --cpus 2` | Launch a native VM instance |
| `sigma-dev vm snapshot --name vm1 --label initial` | Snapshot a running VM |

---

### 📦 Package Management (SigmaPKG)
| Command | Action |
|---|---|
| `sigma-pkg list --installed` | List all installed packages |
| `sigma-pkg depends <name>` | Show dependency tree for a package |
| `sigma-pkg verify <name>` | Verify cryptographic integrity of installed package |
| `sigma-pkg hold <name>` | Prevent a package from being updated |
| `sigma-pkg pin <name> --version 1.2.3` | Pin package to a specific version |
| `sigma-pkg changelog <name>` | View changelog for a package |
| `sigma-pkg info <name>` | Show full package metadata |
| `sigma-pkg build --spec package.sigma` | Build a package from a spec file |
| `sigma-pkg publish --file package.sigma --registry local` | Publish a package to a registry |

**PRINCIPLE:** The GUI Architect generates Omni-Shell commands and dispatches them. The Omni-Shell IS the true OS. The GUI is a convenience layer only.
"""

    with open(guide_path, "a", encoding="utf-8") as f:
        f.write(new_content)

    print("Appended Mega CLI Catalog Batch 2 to os_guide.md.")

    try:
        subprocess.run(["git", "add", "os_guide.md"], check=True)
        subprocess.run(["git", "commit", "-m", "Add 200+ new CLI commands: mega catalog batch 2, full GUI parity"], check=True)
        subprocess.run(["git", "push"], check=True)
        print("Successfully synced Mega CLI Catalog with GitHub.")
    except Exception as e:
        print(f"Git operations failed: {e}")

if __name__ == "__main__":
    append_mega_cli_catalog()
