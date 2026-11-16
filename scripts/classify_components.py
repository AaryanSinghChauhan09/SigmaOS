#!/usr/bin/env python3
"""Classify SigmaOS components with engine='unknown' / category='Unclassified'.

Deterministic keyword classifier: maps component names to engine + category
using ordered rule tables. Rewrites COMPONENTS.toml in place and prints a
coverage report. Safe to re-run; already-classified entries are untouched.
"""
import re
import sys
from pathlib import Path

TOML = Path(__file__).resolve().parent.parent / "COMPONENTS.toml"

# Ordered (engine, category, keywords) rules — first match wins.
# Keywords matched against the lowercase hyphenated component suffix.
RULES = [
    ("kernel_core", "Kernel", ["kernel", "scheduler", "syscall", "vfs", "memory"]),
    ("boot", "Boot", ["boot", "bootloader", "initramfs", "grub", "uefi"]),
    ("init_system", "System Services", ["init", "systemd", "openrc", "runit", "s6", "dinit", "shepherd", "sysvinit", "busybox-init", "dumb-init", "epoch", "finit", "upstart", "66"]),
    ("package_manager", "Package Management", ["pacman", "apt", "dnf", "yum", "zypper", "pkg", "package", "aur", "makepkg", "portage", "dpkg", "rpm", "snap", "flatpak"]),
    ("shell", "Shell", ["shell", "bash", "zsh", "fish", "dash", "ksh", "sh-", "terminal", "pty", "tty"]),
    ("desktop_env", "Desktop Environment", ["gnome", "kde", "plasma", "xfce", "lxqt", "mate", "cinnamon", "budgie", "pantheon", "hyprland", "sway", "wayfire"]),
    ("window_manager", "Window Manager", ["window-manager", "kwin", "mutter", "openbox", "i3", "bspwm", "dwm", "awesome", "xmonad"]),
    ("display_server", "Display Server", ["wayland", "x11", "xorg", "xwayland", "display-server", "windowing"]),
    ("display_manager", "Display Manager", ["gdm", "lightdm", "sddm", "display-manager", "login-manager"]),
    ("file_manager", "File Manager", ["file-manager", "dolphin", "nautilus", "thunar", "nemo", "pcmanfm", "ranger", "yazi", "nnn", "mc", "vifm"]),
    ("text_editor", "Text Editors", ["editor", "vim", "nvim", "emacs", "nano", "code", "sublime", "atom", "kate", "gedit", "mousepad", "leafpad", "helix", "kakoune"]),
    ("terminal_emulator", "Terminal", ["terminal", "alacritty", "kitty", "wezterm", "gnome-terminal", "konsole", "xterm", "foot", "st-"]),
    ("browser", "Browser", ["browser", "chromium", "firefox", "brave", "vivaldi", "opera", "waterfox", "librewolf", "floorp", "zen-browser", "arc-browser", "ungoogled", "epiphany", "falkon", "qutebrowser"]),
    ("media_player", "Multimedia", ["vlc", "mpv", "player", "audacity", "kdenlive", "shotcut", "openshot", "pitivi", "flowblade", "handbrake", "obs", "ffmpeg", "multimedia", "video", "audio", "music"]),
    ("image_graphics", "Graphics", ["gimp", "inkscape", "krita", "blender", "graphics", "photo", "image", "draw", "paint", "svg", "raster", "3d", "cad", "mesh", "render"]),
    ("office", "Office", ["office", "libreoffice", "onlyoffice", "word", "calc", "impress", "writer", "spreadsheet", "presentation", "document", "pdf", "notebook"]),
    ("database", "Databases", ["database", "db", "sql", "postgres", "mysql", "mariadb", "sqlite", "redis", "mongodb", "duckdb", "dynamodb", "cassandra", "clickhouse", "influxdb", "neo4j", "elastic", "solr", "druid", "hbase", "cockroach", "scylla", "rethinkdb", "couchdb", "firestore", "cosmosdb", "simpledb", "rds", "documentdb", "neptune", "timestream", "keyspaces", "memorydb", "redshift", "athena", "quicksight", "glue", "emr", "kinesis", "datapipeline"]),
    ("web_server", "Web Services", ["httpd", "nginx", "apache", "web-server", "caddy", "lighttpd", "traefik", "envoy", "gateway", "proxy", "cdn", "cloudfront"]),
    ("network", "Networking", ["network", "dns", "dhcp", "vpn", "wireguard", "openvpn", "iptables", "nftables", "firewall", "router", "bridge", "wifi", "wireless", "bluetooth", "ethernet", "tcp", "udp", "quic", "stun", "rtp", "rtcp", "sip", "xmpp", "matrix", "irc", "ntp", "snmp", "radius", "ldap", "kerberos", "oauth", "saml", "oidc"]),
    ("security", "Security", ["security", "auth", "crypto", "encrypt", "decrypt", "ssl", "tls", "pki", "certificate", "cert", "key", "vault", "secret", "token", "password", "passwd", "sudo", "pam", "selinux", "apparmor", "seccomp", "capability", "audit", "forensics", "antivirus", "malware", "firewall2", "ids", "ips", "waf", "siem", "soc", "threat", "vuln", "cve", "fuzz", "hardening", "sandbox", "capsicum", "pledge", "signify", "sshd", "openssh", "gnupg", "gpg", "sops", "age", "minisign"]),
    ("filesystem", "Filesystems", ["filesystem", "fs", "ext4", "btrfs", "xfs", "zfs", "ntfs", "fat", "exfat", "iso9660", "udf", "f2fs", "nilfs", "reiser", "jfs", "hfs", "apfs", "overlay", "fuse", "mount", "partition", "lvm", "raid", "mdadm", "device-mapper", "loop", "nfs", "cifs", "smb", "sshfs", "davfs", "unionfs"]),
    ("storage", "Storage", ["storage", "disk", "drive", "volume", "backup", "archive", "snapshot", "sync", "rsync", "borg", "restic", "duplicity", "tar", "zip", "gzip", "xz", "zstd", "lz4", "7z", "compression", "dedup", "tiering", "cache", "buffer", "queue", "stream", "log", "journal", "syslog"]),
    ("virtualization", "Virtualization", ["vm", "virtual", "qemu", "kvm", "xen", "hypervisor", "virtualbox", "vmware", "docker", "container", "podman", "lxc", "lxd", "incus", "wsl", "waydroid", "emulator", "vagrant", "ansible", "terraform", "packer", "vagrant", "multipass", "firecracker", "cloud-hypervisor", "kata", "gvisor"]),
    ("compiler_toolchain", "Compilers", ["gcc", "clang", "llvm", "rustc", "go", "javac", "gfortran", "compiler", "toolchain", "binutils", "as", "ld", "ar", "nm", "objdump", "make", "cmake", "ninja", "meson", "bazel", "buck", "gradle", "maven", "sbt", "cargo", "rustup", "cross", "musl", "glibc", "dietlibc", "uclibc"]),
    ("language_runtime", "Languages", ["python", "node", "nodejs", "deno", "bun", "ruby", "perl", "php", "lua", "tcl", "java", "kotlin", "scala", "groovy", "csharp", "dotnet", "fsharp", "swift", "rust", "golang", "haskell", "ocaml", "erlang", "elixir", "clojure", "julia", "r-lang", "matlab", "octave", "nim", "zig", "crystal", "dart", "flutter", "v8", "quickjs", "duktape", "chakra", "spidermonkey", "javascriptcore", "wasm", "webassembly"]),
    ("gui_toolkit", "GUI Frameworks", ["gtk", "qt", "sdl", "wx", "tk", "electron", "tauri", "flutter", "kivy", "flet", "imgui", "wgpu", "vulkan", "opengl", "glfw", "sfml", "raylib", "love2d", "godot", "unity", "unreal", "cocos", "defold", "monogame", "xamarin", "avalonia", "uno", "react-native", "nativegui", "fresco", "awt", "swing", "javafx", "wpf", "winforms", "uwp", "qt6", "gtk4", "gtk3", "qt5", "qtquick", "qml"]),
    ("windowing_protocol", "Display", ["wayland-protocol", "xdg", "zwlr", "layer-shell", "input-method", "im", "ibus", "fcitx", "scim", "uinput", "evdev", "libinput", "drm", "gbm", "egl", "glx", "mesa", "vulkan-loader"]),
    ("service_daemon", "System Services", ["daemon", "service", "server", "agent", "watcher", "monitor", "cron", "timer", "scheduler", "queue", "worker", "runner", "executor", "supervisor", "respawn", "guardian", "sentinel"]),
    ("user_management", "User Management", ["user", "users", "group", "groups", "passwd", "shadow", "login", "logout", "session", "seat", "polkit", "accounts", "identity", "sso", "2fa", "mfa", "totp", "webauthn", "fido", "keyring", "gnome-keyring", "kwallet", "seahorse"]),
    ("hardware_driver", "Device Drivers", ["driver", "drivers", "firmware", "bios", "uefi2", "acpi", "pci", "usb", "scsi", "sata", "nvme", "gpu", "drm2", "sound", "alsa", "pulseaudio", "pipewire", "video4linux", "v4l", "wifi-driver", "bluetooth-driver", "serial", "parallel", "i2c", "spi", "gpio", "pwm", "watchdog", "rtc", "hwmon", "leds", "input", "hid", "touch", "tablet", "joystick", "camera", "webcam", "printer", "scanner", "modem", "ups", "lm-sensors", "cpufreq", "cpuidle", "msr", "microcode"]),
    ("system_library", "System Libraries", ["lib", "glib", "dbus", "systemd-lib", "libc", "libm", "libpthread", "librt", "libdl", "libnss", "libcrypt", "libcurl", "zlib", "openssl", "icu", "pcre", "ncurses", "readline", "ffi", "libxml", "libxslt", "sqlite-lib", "expat", "jpeg", "png", "freetype", "fontconfig", "cairo", "pango", "harfbuzz", "fribidi", "icu4c", "boost", "eigen", "opencv", "vtk", "itk", "gsl", "range-v3", "fmt", "spdlog", "abseil", "protobuf", "grpc", "thrift", "avro", "capnp", "flatbuffers", "msgpack", "cbor", "bson", "yaml", "toml", "ini", "json", "xml", "html", "css", "dom", "sax"]),
    ("build_ci", "DevOps", ["ci", "cd", "jenkins", "gitlab-ci", "github-actions", "circleci", "travis", "buildkite", "drone", "concourse", "tekton", "argo", "flux", "spinnaker", "harness", "teamcity", "bamboo", "codeship", "semaphore", "appveyor", "azure-pipelines", "build", "deploy", "release", "artifact", "registry", "harbor", "nexus", "artifactory", "verdaccio", "docker-registry", "helm", "kustomize", "jsonnet", "skaffold", "tilt", "devspace", "garden", "bazel2", "distcc", "ccache", "icecc"]),
    ("vcs", "Version Control", ["git", "svn", "hg", "mercurial", "fossil", "bzr", "darcs", "cvs", "repo", "gerrit", "gitlab", "github", "bitbucket", "sourcehut", "codeberg", "git-town", "git-flow", "lfs", "submodule", "worktree", "rebase", "cherry-pick", "bisect"]),
    ("testing", "Testing", ["test", "tests", "testing", "unit", "integration", "e2e", "benchmark", "bench", "coverage", "codecov", "coveralls", "jest", "mocha", "pytest", "junit", "testng", "rspec", "cucumber", "selenium", "playwright", "cypress", "puppeteer", "karma", "jasmine", "ava", "tap", "proptest", "quickcheck", "hypothesis", "fuzz", "afl", "honggfuzz", "libfuzzer", "oss-fuzz", "clusterfuzz", "sanitizer", "asan", "ubsan", "msan", "tsan", "lsan"]),
    ("debugging", "Debugging", ["debug", "debugger", "gdb", "lldb", "strace", "ltrace", "perf", "valgrind", "addr2line", "objdump2", "readelf", "nm2", "eu-stack", "pstack", "crash", "coredump", "backtrace", "trace", "tracing", "telemetry", "observability", "opentelemetry", "jaeger", "zipkin", "prometheus", "grafana", "alertmanager", "thanos", "cortex", "victoria", "influx", "telegraf", "collectd", "statsd", "datadog", "newrelic", "dynatrace", "appdynamics", "instana", "honeycomb", "lightstep", "signalfx", "wavefront", "sumologic", "splunk", "elastic2", "kibana", "logstash", "filebeat", "metricbeat", "packetbeat", "heartbeat", "auditbeat", "winlogbeat"]),
    ("documentation", "Documentation", ["doc", "docs", "documentation", "wiki", "readme", "man", "info", "help", "guide", "tutorial", "handbook", "book", "spec", "specification", "adr", "changelog", "release-notes"]),
    ("internationalization", "i18n", ["i18n", "l10n", "locale", "locales", "translation", "gettext", "icu2", "cldr", "lang", "language", "font", "fonts", "unicode", "utf", "encoding", "charset", "collation", "plural", "gender"]),
    ("accessibility", "Accessibility", ["accessibility", "a11y", "screen-reader", "orca", "magnifier", "high-contrast", "colorblind", "tts", "speech", "voice", "narator", "magnifier", "on-screen-keyboard", "osk", "keyboard", "input-method"]),
    ("printing", "Printing", ["print", "printer", "printers", "cups", "ipp", "lpr", "lpadmin", "ghostscript", "mupdf", "poppler", "evince", "okular", "zathura", "bookreader", "comics", "manga", "cbz"]),
    ("game", "Games", ["game", "games", "gaming", "emulator", "retroarch", "dosbox", "scummvm", "wine", "proton", "lutris", "heroic", "playnite", "steam", "epic", "gog", "itch", "roblox", "minecraft", "unity-hub", "unreal-engine", "godot2", "pygame", "sdl-game", "arcade", "mame", "fceux", "mednafen", "pcsx2", "rpcs3", "yuzu", "ryujinx", "dolphin-emu", "citra", "ppsspp", "xemu", "duckstation", "ares", "mgba", "visualboyadvance", "desmume", "melonDS".lower()]),
    ("iot", "IoT", ["iot", "esp", "arduino", "raspberry", "gpio", "zigbee", "zwave", "lora", "sigfox", "nbiot", "matter", "thread", "homekit", "home-assistant", "openhab", "domoticz", "sensor", "actuator", "mqtt", "coap", "modbus", "opc", "bacnet", "knx", "dali", "dmx", "can", "canbus", "obd", "telematics"]),
    ("robotics", "Robotics", ["robot", "robotics", "ros", "ros2", "drone", "uav", "rosbridge", "moveit", "gazebo", "ignition", "nav2", "slam", "amcl", "cartographer", "rtab", "octomap", "costmap", "tf2", "urdf", "sdf", "xacro", "rviz", "rqt"]),
    ("scientific", "Science", ["science", "scientific", "math", "mathematics", "physics", "chemistry", "biology", "bioinformatics", "genomics", "protein", "molecule", "quantum", "simulation", "simulink", "matlab2", "octave2", "scilab", "jupyter", "notebook", "ipython", "matplotlib", "pandas", "numpy", "scipy", "sklearn", "tensorflow", "pytorch", "keras", "jax", "mxnet", "caffe", "theano", "paddle", "onnx", "tensorrt", "triton", "data-science", "datascience", "ml", "ai", "deeplearning", "machine-learning", "nlp", "cv", "computer-vision", "ocr", "asr", "tts", "speech", "translation", "embeddings", "vector", "faiss", "annoy", "hnsw", "milvus", "weaviate", "pinecone", "qdrant", "chromadb", "pgvector"]),
    ("data_engineering", "Data Engineering", ["spark", "hadoop", "hive", "pig", "flink", "storm", "kafka", "pulsar", "kinesis", "beam", "airflow", "luigi", "prefect", "dagster", "dbt", "fivetran", "airbyte", "segment", "snowplow", "etl", "elt", "ingestion", "pipeline", "warehouse", "lake", "datalake", "lakehouse", "delta", "iceberg", "hudi", "parquet", "avro2", "orc", "arrow", "hudi", "hbase2", "presto", "trino", "drill", "impala", "phoenix", "kyuubi", "spark2", "flink2", "beam2", "samza", "heron", "apex", "nifi", "streamsets", "talend", "pentaho", "informatica", "fivetran2", "matillion", "stitch", "hevo", "rivery", "sling", "meltano", "singer", "rudderstack", "jitsu", "grouparoo", "hightouch", "census", "polytomic", "polytomic", "dataddo", "portable", "panoply", "fishtown", "transform", "looker", "tableau", "powerbi", "superset", "metabase", "redash", "mode", "periscope", "chartio", "databox", "grafana2", "superset2", "evidence", "rill", "hex", "deepnote", "observable", "jupyter2", "zeppelin", "h2o", "databricks", "snowflake", "bigquery", "redshift2", "synapse", "fabric", "motherduck", "clickhouse2", "druid2", "pinot", "doris", "starrocks", "trino2", "presto2", "athena2", "emr2", "glue2", "dataproc", "hdinsight", "synapse2", "fabric2", "lakehouse2"]),
    ("cloud_provider", "Cloud", ["aws", "azure", "gcp", "google-cloud", "alibaba", "oracle-cloud", "ibm-cloud", "digitalocean", "linode", "vultr", "hetzner", "ovh", "scaleway", "upcloud", "kamatera", "cloudflare", "fastly", "akamai", "vercel", "netlify", "heroku", "render", "fly.io", "railway", "replit", "codesandbox", "stackblitz", "gitpod", "coder", "devpod", "gitpod2", "daytona", "coder2", "jupyterhub", "binder", "mybinder", "google-colab", "colab", "sagemaker", "bedrock", "vertex", "azureml", "azure-ml", "azure-ai", "aws-ai", "gcp-ai", "vertex-ai", "sagemaker-ai", "bedrock-ai", "cognito", "iam", "sts", "kms", "secrets-manager", "parameter-store", "ssm", "cloudwatch", "cloudtrail", "config", "guardduty", "securityhub", "inspector", "macie", "detective", "audit-manager", "artifact", "artifact2", "codeartifact", "ecr", "ecr2", "ecs", "eks", "fargate", "lambda", "lambda2", "serverless", "apigateway", "api-gateway", "appsync", "amplify", "app-runner", "elastic-beanstalk", "beanstalk", "opsworks", "cloudformation", "cdk", "sam", "serverless2", "terraform2", "pulumi", "crossplane", "cdktf", "terragrunt", "atlantis", "env0", "spacelift", "scalr", "brainboard", "former2", "duplocloud", "infracost", "checkov", "terrascan", "tfsec", "kics", "trivy", "grype", "syft", "cosign", "notation", "ratify", "in-toto", "slsa", "sigstore", "fulcio", "rekor", "canary", "flagger", "argo-rollouts", "keptn", "openfeature", "flagsmith", "unleash", "launchdarkly", "split", "optimizely", "growthbook", "posthog", "amplitude", "mixpanel", "heap", "fullstory", "hotjar", "mouseflow", "smartlook", "logrocket", "sentry", "bugsnag", "rollbar", "airbrake", "honeybadger", "raygun", "trackjs", "loggly", "papertrail", "logentries", "logz", "mezmo", "highlight", "baselime", "axiom", "betterstack", "checkly", "pingdom", "uptimerobot", "statuscake", "gatus", "statping", "cachet", "status.io", "sorryapp"]),
    ("messaging", "Messaging", ["kafka2", "rabbitmq", "activemq", "artemis", "nats", "zeromq", "nanomsg", "mqtt", "amqp", "stomp", "jms", "redis2", "memcached", "valkey", "keydb", "dragonfly", "ssdb", "ledisdb", "hazelcast", "infinispan", "coherence", "gemfire", "geode", "ignite", "hazelcast2", "voltdb", "memsql", "singlestore", "tidb", "vitess", "yugabyte", "cockroachdb", "tidb2", "oceanbase", "polardb", "gaussdb", "opengauss", "mogdb", "vastbase", "highgo", "oscar", "uxdb", "vastbase2", "mogdb2", "kunpeng", "kylin", "anolis", "openEuler", "opencloudos", "tencentos", "alinux", "openeuler2", "opencloudos2", "deepin", "ubuntukylin", "startos", "uos", "neokylin", "kylinos", "turbolinux", "lindows", "xandros", "linspire", "mandrake", "mandriva", "pclinuxos", "mageia", "rosa", "altlinux", "astra", "redos", "calculate", "calculate2", "funtoo", "artix", "void", "devuan", "antiX", "mxlinux", "peppermint", "lubuntu", "xubuntu", "kubuntu", "lubuntu2", "zorin", "pop", "pop-os", "elementary", "deepin2", "manjaro", "endeavouros", "garuda", "archcraft", "arch", "cachyos", "omarchy", "omacom", "sigmaos", "nixos", "guix", "gobo", "bower", "exherbo", "paludis", "equo", "spack", "easybuild", "lmod", "module", "modules", "environment", "env", "venv", "virtualenv", "conda2", "mamba", "micromamba", "pixi", "poetry", "pipenv", "hatch", "pdm", "uv", "rye", "flit", "pbr", "setuptools", "wheel", "twine", "build2", "installer", "installer2", "packaging", "distribution", "dist", "release2", "versioning", "semver", "calver", "pep440", "npm2", "yarn", "pnpm", "bun2", "deno2", "corepack", "volta", "nvm", "n", "fnm", "asdf", "mise", "rtx", "proto", "aqua", "devbox", "devenv", "flox", "nixpacks", "buildpacks", "paketo", "heroku2", "dokku", "caprover", "coolify", "dokploy", " easypanel", "cosmic", "cosmos", "caprover2", "tsuru", "flynn", "deis", "helm2", "kustomize2", "argocd", "flux2", "argo-cd", "jenkins2", "jenkins-x", "tekton2", "spinnaker2", "harness2", "circleci2", "travis2", "drone2", "concourse2", "teamcity2", "bamboo2", "codeship2", "semaphore2", "appveyor2", "azure-pipelines2", "github-actions2", "gitlab-ci2", "buildkite2", "ci", "cd2"]),
]

# engine/category used when no rule matches
DEFAULT = ("unknown", "Unclassified")

# Curated overrides for names that keyword rules cannot classify safely.
# Reviewed individually; engine values reuse existing registry vocabulary.
OVERRIDES = {
    "0ad": ("game_engine", "Games"),
    "7-days-to-die": ("game_engine", "Games"),
    "action-center": ("desktop_app", "Desktop"),
    "active-directory": ("directory_service", "User Management"),
    "adwaita": ("theme_engine", "Desktop"),
    "affy": ("language_runtime", "Languages"),
    "ahci": ("hardware_driver", "Device Drivers"),
    "aide": ("security_tool", "Security"),
    "aiops": ("observability", "Debugging"),
    "akka": ("language_runtime", "Languages"),
    "akmod": ("kernel_module", "Kernel"),
    "akonadi": ("desktop_service", "System Services"),
    "albert": ("desktop_app", "Desktop"),
    "alerting": ("observability", "Debugging"),
    "alias": ("shell_tool", "Shell"),
    "allocation": ("memory_subsystem", "Kernel"),
    "alpine": ("distribution", "Linux"),
    "alpm-hooks": ("package_manager", "Package Management"),
    "alpm-rs": ("package_manager", "Package Management"),
    "amanda": ("backup_tool", "Storage"),
    "amarok": ("media_player", "Multimedia"),
    "amavisd": ("security_tool", "Security"),
    "amazon-prime-dl": ("media_tool", "Multimedia"),
    "amber": ("window_manager", "Window Manager"),
    "amd": ("hardware_driver", "Device Drivers"),
    "amd-psp": ("hardware_driver", "Device Drivers"),
    "amdgpu": ("hardware_driver", "Device Drivers"),
    "amf": ("hardware_driver", "Device Drivers"),
    "analytics": ("data_engineering", "Data Engineering"),
    "android-ndk": ("language_runtime", "Languages"),
    "android-sdk": ("language_runtime", "Languages"),
    "android-studio": ("ide", "Development Tools"),
    "animation": ("graphics_lib", "Graphics"),
    "anjuta": ("ide", "Development Tools"),
    "anomaly-detection": ("ml_framework", "Science"),
    "ant": ("build_tool", "DevOps"),
    "anti-pattern": ("documentation", "Documentation"),
    "antora": ("documentation", "Documentation"),
    "anydesk": ("remote_desktop", "Remote Access"),
    "anyrun": ("launcher", "Desktop"),
    "api": ("api_framework", "Web Services"),
    "api-client": ("api_framework", "Web Services"),
    "apiary": ("api_framework", "Web Services"),
    "apm": ("observability", "Debugging"),
    "apollo": ("api_framework", "Web Services"),
    "app-center": ("app_store", "Applications"),
    "app-launcher": ("launcher", "Desktop"),
}


# Keywords that are common English substrings — token match only.
RISKY_TOKENS = {
    "test", "tests", "doc", "docs", "git", "run", "art", "net", "web",
    "app", "apps", "dev", "ops", "sec", "lab", "hub", "pro", "max",
    "bot", "ui", "ux", "ide", "sdk", "api", "cli", "gui", "tui",
    "sql", "css", "html", "xml", "r", "c", "go", "rb", "js", "ts",
    "sh", "ps", "bat", "cmd", "asm", "bin", "lib", "src", "doc2",
}


def _kw_match(kw: str, n: str) -> bool:
    """Short/common-substring keywords must match a whole hyphen-delimited
    token to avoid false positives (e.g. 'ai' in 'domain', 'ml' in 'html',
    'test' in 'latest')."""
    if len(kw) <= 3 or kw in RISKY_TOKENS:
        return kw in n.split("-")
    return kw in n


def classify(name: str):
    """Return (engine, category) for a SigmaOS-<name> component."""
    n = name.lower()
    if n.startswith("sigmaos-"):
        n = n[len("sigmaos-"):]
    if n in OVERRIDES:
        return OVERRIDES[n]
    for engine, category, kws in RULES:
        for kw in kws:
            if _kw_match(kw, n):
                return engine, category
    return DEFAULT


def main() -> int:
    apply = "--apply" in sys.argv
    text = TOML.read_text()
    blocks = re.split(r"(?=\[\[component\]\])", text)
    counts = {}
    changed = 0
    out = []
    for b in blocks:
        m = re.search(r'^name = "([^"]+)"', b, re.M)
        if not m:
            out.append(b)
            continue
        name = m.group(1)
        if 'engine = "unknown"' in b and 'category = "Unclassified"' in b:
            engine, cat = classify(name)
            counts[(engine, cat)] = counts.get((engine, cat), 0) + 1
            if (engine, cat) != DEFAULT:
                changed += 1
                if apply:
                    b = b.replace('engine = "unknown"', f'engine = "{engine}"')
                    b = b.replace('category = "Unclassified"', f'category = "{cat}"')
        out.append(b)
    if apply:
        TOML.write_text("".join(out))
    total_unknown = sum(1 for b in blocks if 'engine = "unknown"' in b)
    print(f"components scanned: {sum(1 for b in blocks if '[[component]]' in b)}")
    print(f"unknown/unclassified blocks: {total_unknown}")
    print(f"newly classified: {changed} (apply={apply})")
    for (e, c), n in sorted(counts.items(), key=lambda x: -x[1])[:20]:
        print(f"  {e:20s} {c:25s} {n}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
