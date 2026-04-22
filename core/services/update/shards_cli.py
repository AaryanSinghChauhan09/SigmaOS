import argparse
import sys
import os

# The "Sovereign Shards CLI" Package Manager
# Handles fetching, verifying, and installing WASM payloads

MOCK_REGISTRY = {
    "office-suite": "v1.2.0 - Sovereign WASM Doc Editor",
    "vim-shard": "v8.2.0 - Core Terminal Editor",
    "wireguard-gui": "v0.5.1 - Networking Overlay",
    "zenith-metrics": "v2.0.0 - Advanced Task Monitor"
}

def install_shard(name):
    if name not in MOCK_REGISTRY:
        print(f"❌ [Error] Shard '{name}' not found in the Sovereign Registry.")
        sys.exit(1)
        
    print(f"🔍 Locating '{name}' in registry...")
    print(f"⬇️ Downloading bytecode for {MOCK_REGISTRY[name]}...")
    print("🛡️ Verifying cryptographic signature via core/security...")
    
    # Simulate writing the WASM payload into the plugins directory
    print(f"📦 Unpacking WASM sandbox constraints for {name}...")
    print(f"✅ Success! Shard '{name}' loaded into the Sovereign Lattice.\n")

def list_shards():
    print("--- 🌐 Sovereign Remote Registry (Available Modules) ---")
    for shard, desc in MOCK_REGISTRY.items():
        print(f"  > {shard:<18} : {desc}")

def main():
    parser = argparse.ArgumentParser(description="SigmaOS Native Package Manager (shards-cli)")
    subparsers = parser.add_subparsers(dest="command")

    # Install Command
    install_parser = subparsers.add_parser("install", help="Install a new WASM shard")
    install_parser.add_argument("shard_name", help="Name of the shard to install")

    # List Command
    subparsers.add_parser("list", help="List available shards in remote registry")

    args = parser.parse_args()

    if args.command == "install":
        install_shard(args.shard_name)
    elif args.command == "list":
        list_shards()
    else:
        parser.print_help()

if __name__ == "__main__":
    main()
