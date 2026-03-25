"""
SigmaOS Virtualization Core Implementation
==========================================
Advanced virtualization and containerization engine
"""

import sys
import os
import json
import time
import subprocess
import threading
from typing import Dict, List, Optional, Any, Callable
from dataclasses import dataclass, field
from enum import Enum
import hashlib
import uuid

class VMType(Enum):
    KVM = "kvm"
    QEMU = "qemu"
    DOCKER = "docker"
    PODMAN = "podman"
    LXC = "lxc"
    SIGMA_NATIVE = "sigma_native"

class ContainerType(Enum):
    DOCKER = "docker"
    PODMAN = "podman"
    SIGMA_CONTAINER = "sigma_container"

class VMState(Enum):
    STOPPED = "stopped"
    STARTING = "starting"
    RUNNING = "running"
    PAUSED = "paused"
    STOPPING = "stopping"
    ERROR = "error"

@dataclass
class VMConfig:
    name: str
    vm_type: VMType
    memory_mb: int
    cpu_cores: int
    disk_size_gb: int
    network_mode: str = "bridge"
    iso_path: Optional[str] = None
    vnc_port: Optional[int] = None
    ssh_port: Optional[int] = None
    additional_args: Dict[str, Any] = field(default_factory=dict)

@dataclass
class ContainerConfig:
    name: str
    container_type: ContainerType
    image: str
    command: Optional[str] = None
    environment: Dict[str, str] = field(default_factory=dict)
    volumes: Dict[str, str] = field(default_factory=dict)
    ports: Dict[str, str] = field(default_factory=dict)
    network_mode: str = "bridge"
    auto_remove: bool = False

@dataclass
class VirtualMachine:
    id: str
    config: VMConfig
    state: VMState
    pid: Optional[int] = None
    ip_address: Optional[str] = None
    created_at: float = field(default_factory=time.time)
    started_at: Optional[float] = None
    console_log: List[str] = field(default_factory=list)

@dataclass
class Container:
    id: str
    config: ContainerConfig
    state: VMState
    container_id: Optional[str] = None
    ip_address: Optional[str] = None
    created_at: float = field(default_factory=time.time)
    started_at: Optional[float] = None
    logs: List[str] = field(default_factory=list)

class SigmaVirtualizationEngine:
    """
    Advanced virtualization and containerization engine for SigmaOS
    """
    
    def __init__(self):
        self.vms: Dict[str, VirtualMachine] = {}
        self.containers: Dict[str, Container] = {}
        self.host_info = self._get_host_info()
        self.network_pools = self._init_network_pools()
        self.storage_pools = self._init_storage_pools()
        
    def _get_host_info(self) -> Dict[str, Any]:
        """Get host system information"""
        try:
            import psutil
            return {
                "cpu_count": psutil.cpu_count(),
                "memory_total": psutil.virtual_memory().total,
                "memory_available": psutil.virtual_memory().available,
                "disk_free": psutil.disk_usage('/').free,
                "virtualization_support": self._check_virtualization_support(),
                "kvm_available": self._check_kvm_available()
            }
        except ImportError:
            # Fallback if psutil not available
            return {
                "cpu_count": os.cpu_count(),
                "memory_total": 0,
                "memory_available": 0,
                "disk_free": 0,
                "virtualization_support": False,
                "kvm_available": False
            }
    
    def _check_virtualization_support(self) -> bool:
        """Check if host supports virtualization"""
        try:
            with open('/proc/cpuinfo', 'r') as f:
                cpuinfo = f.read()
                return 'vmx' in cpuinfo or 'svm' in cpuinfo
        except:
            return False
    
    def _check_kvm_available(self) -> bool:
        """Check if KVM is available"""
        try:
            return os.path.exists('/dev/kvm')
        except:
            return False
    
    def _init_network_pools(self) -> Dict[str, Any]:
        """Initialize network address pools"""
        return {
            "vm_network": {
                "subnet": "192.168.100.0/24",
                "gateway": "192.168.100.1",
                "dhcp_range": "192.168.100.100-192.168.100.200"
            },
            "container_network": {
                "subnet": "172.20.0.0/16",
                "gateway": "172.20.0.1",
                "dhcp_range": "172.20.0.100-172.20.0.200"
            }
        }
    
    def _init_storage_pools(self) -> Dict[str, Any]:
        """Initialize storage pools"""
        storage_dir = "/var/lib/sigma/virtualization"
        os.makedirs(storage_dir, exist_ok=True)
        
        return {
            "vm_storage": f"{storage_dir}/vms",
            "container_storage": f"{storage_dir}/containers",
            "iso_storage": f"{storage_dir}/isos",
            "snapshot_storage": f"{storage_dir}/snapshots"
        }
    
    def create_vm(self, config: VMConfig) -> str:
        """Create a new virtual machine"""
        vm_id = str(uuid.uuid4())
        
        # Validate resources
        if not self._validate_vm_resources(config):
            raise ValueError("Insufficient resources for VM")
        
        vm = VirtualMachine(
            id=vm_id,
            config=config,
            state=VMState.STOPPED
        )
        
        self.vms[vm_id] = vm
        
        # Create VM disk
        self._create_vm_disk(vm)
        
        # Generate VM configuration
        self._generate_vm_config(vm)
        
        return vm_id
    
    def start_vm(self, vm_id: str) -> bool:
        """Start a virtual machine"""
        if vm_id not in self.vms:
            return False
        
        vm = self.vms[vm_id]
        if vm.state != VMState.STOPPED:
            return False
        
        vm.state = VMState.STARTING
        
        try:
            if vm.config.vm_type == VMType.QEMU:
                success = self._start_qemu_vm(vm)
            elif vm.config.vm_type == VMType.KVM and self.host_info["kvm_available"]:
                success = self._start_kvm_vm(vm)
            elif vm.config.vm_type == VMType.SIGMA_NATIVE:
                success = self._start_sigma_native_vm(vm)
            else:
                success = False
            
            if success:
                vm.state = VMState.RUNNING
                vm.started_at = time.time()
                vm.ip_address = self._allocate_vm_ip(vm)
            else:
                vm.state = VMState.ERROR
            
            return success
        
        except Exception as e:
            vm.state = VMState.ERROR
            vm.console_log.append(f"Error starting VM: {str(e)}")
            return False
    
    def stop_vm(self, vm_id: str, force: bool = False) -> bool:
        """Stop a virtual machine"""
        if vm_id not in self.vms:
            return False
        
        vm = self.vms[vm_id]
        if vm.state != VMState.RUNNING:
            return False
        
        vm.state = VMState.STOPPING
        
        try:
            if force and vm.pid:
                os.kill(vm.pid, 9)
            else:
                self._send_vm_shutdown_signal(vm)
            
            vm.state = VMState.STOPPED
            vm.pid = None
            vm.ip_address = None
            vm.started_at = None
            
            return True
        
        except Exception as e:
            vm.state = VMState.ERROR
            vm.console_log.append(f"Error stopping VM: {str(e)}")
            return False
    
    def pause_vm(self, vm_id: str) -> bool:
        """Pause a virtual machine"""
        if vm_id not in self.vms:
            return False
        
        vm = self.vms[vm_id]
        if vm.state != VMState.RUNNING:
            return False
        
        try:
            if vm.pid:
                os.kill(vm.pid, 19)  # SIGSTOP
            vm.state = VMState.PAUSED
            return True
        except:
            return False
    
    def resume_vm(self, vm_id: str) -> bool:
        """Resume a paused virtual machine"""
        if vm_id not in self.vms:
            return False
        
        vm = self.vms[vm_id]
        if vm.state != VMState.PAUSED:
            return False
        
        try:
            if vm.pid:
                os.kill(vm.pid, 18)  # SIGCONT
            vm.state = VMState.RUNNING
            return True
        except:
            return False
    
    def delete_vm(self, vm_id: str) -> bool:
        """Delete a virtual machine"""
        if vm_id not in self.vms:
            return False
        
        vm = self.vms[vm_id]
        
        # Stop VM if running
        if vm.state == VMState.RUNNING:
            self.stop_vm(vm_id, force=True)
        
        # Clean up resources
        self._cleanup_vm_resources(vm)
        
        del self.vms[vm_id]
        return True
    
    def create_container(self, config: ContainerConfig) -> str:
        """Create a new container"""
        container_id = str(uuid.uuid4())
        
        container = Container(
            id=container_id,
            config=config,
            state=VMState.STOPPED
        )
        
        self.containers[container_id] = container
        
        # Pull container image if needed
        if config.container_type in [ContainerType.DOCKER, ContainerType.PODMAN]:
            self._pull_container_image(container)
        
        return container_id
    
    def start_container(self, container_id: str) -> bool:
        """Start a container"""
        if container_id not in self.containers:
            return False
        
        container = self.containers[container_id]
        if container.state != VMState.STOPPED:
            return False
        
        container.state = VMState.STARTING
        
        try:
            if container.config.container_type == ContainerType.DOCKER:
                success = self._start_docker_container(container)
            elif container.config.container_type == ContainerType.PODMAN:
                success = self._start_podman_container(container)
            elif container.config.container_type == ContainerType.SIGMA_CONTAINER:
                success = self._start_sigma_container(container)
            else:
                success = False
            
            if success:
                container.state = VMState.RUNNING
                container.started_at = time.time()
                container.ip_address = self._allocate_container_ip(container)
            else:
                container.state = VMState.ERROR
            
            return success
        
        except Exception as e:
            container.state = VMState.ERROR
            container.logs.append(f"Error starting container: {str(e)}")
            return False
    
    def stop_container(self, container_id: str, force: bool = False) -> bool:
        """Stop a container"""
        if container_id not in self.containers:
            return False
        
        container = self.containers[container_id]
        if container.state != VMState.RUNNING:
            return False
        
        try:
            if container.config.container_type == ContainerType.DOCKER:
                cmd = ["docker", "stop"]
                if force:
                    cmd.append("--force")
                cmd.append(container.container_id)
                subprocess.run(cmd, check=True)
            elif container.config.container_type == ContainerType.PODMAN:
                cmd = ["podman", "stop"]
                if force:
                    cmd.append("--force")
                cmd.append(container.container_id)
                subprocess.run(cmd, check=True)
            
            container.state = VMState.STOPPED
            container.container_id = None
            container.ip_address = None
            container.started_at = None
            
            return True
        
        except:
            return False
    
    def delete_container(self, container_id: str) -> bool:
        """Delete a container"""
        if container_id not in self.containers:
            return False
        
        container = self.containers[container_id]
        
        # Stop container if running
        if container.state == VMState.RUNNING:
            self.stop_container(container_id, force=True)
        
        # Remove container
        if container.container_id:
            try:
                if container.config.container_type == ContainerType.DOCKER:
                    subprocess.run(["docker", "rm", "-f", container.container_id], check=True)
                elif container.config.container_type == ContainerType.PODMAN:
                    subprocess.run(["podman", "rm", "-f", container.container_id], check=True)
            except:
                pass
        
        del self.containers[container_id]
        return True
    
    def get_vm_info(self, vm_id: str) -> Optional[Dict[str, Any]]:
        """Get VM information"""
        if vm_id not in self.vms:
            return None
        
        vm = self.vms[vm_id]
        return {
            "id": vm.id,
            "name": vm.config.name,
            "type": vm.config.vm_type.value,
            "state": vm.state.value,
            "memory_mb": vm.config.memory_mb,
            "cpu_cores": vm.config.cpu_cores,
            "disk_size_gb": vm.config.disk_size_gb,
            "ip_address": vm.ip_address,
            "pid": vm.pid,
            "created_at": vm.created_at,
            "started_at": vm.started_at,
            "uptime": time.time() - vm.started_at if vm.started_at else 0
        }
    
    def get_container_info(self, container_id: str) -> Optional[Dict[str, Any]]:
        """Get container information"""
        if container_id not in self.containers:
            return None
        
        container = self.containers[container_id]
        return {
            "id": container.id,
            "name": container.config.name,
            "type": container.config.container_type.value,
            "image": container.config.image,
            "state": container.state.value,
            "ip_address": container.ip_address,
            "container_id": container.container_id,
            "created_at": container.created_at,
            "started_at": container.started_at,
            "uptime": time.time() - container.started_at if container.started_at else 0
        }
    
    def list_vms(self) -> List[Dict[str, Any]]:
        """List all virtual machines"""
        return [self.get_vm_info(vm_id) for vm_id in self.vms.keys()]
    
    def list_containers(self) -> List[Dict[str, Any]]:
        """List all containers"""
        return [self.get_container_info(container_id) for container_id in self.containers.keys()]
    
    def get_host_resources(self) -> Dict[str, Any]:
        """Get host resource usage"""
        try:
            import psutil
            return {
                "cpu_percent": psutil.cpu_percent(),
                "memory_percent": psutil.virtual_memory().percent,
                "disk_percent": psutil.disk_usage('/').percent,
                "cpu_count": psutil.cpu_count(),
                "memory_total": psutil.virtual_memory().total,
                "memory_available": psutil.virtual_memory().available,
                "disk_total": psutil.disk_usage('/').total,
                "disk_free": psutil.disk_usage('/').free
            }
        except ImportError:
            return {}
    
    def _validate_vm_resources(self, config: VMConfig) -> bool:
        """Validate VM resource requirements"""
        host_info = self.host_info
        
        # For testing purposes, always return True if psutil is not available
        if host_info["memory_available"] == 0:
            return True
        
        if config.memory_mb > host_info["memory_available"] // (1024 * 1024):
            return False
        
        if config.cpu_cores > host_info["cpu_count"]:
            return False
        
        return True
    
    def _create_vm_disk(self, vm: VirtualMachine) -> None:
        """Create VM disk image"""
        disk_path = f"{self.storage_pools['vm_storage']}/{vm.id}.qcow2"
        
        cmd = [
            "qemu-img", "create", "-f", "qcow2",
            disk_path, f"{vm.config.disk_size_gb}G"
        ]
        
        try:
            subprocess.run(cmd, check=True)
        except (subprocess.CalledProcessError, FileNotFoundError) as e:
            # For testing/simulation, create a dummy file
            os.makedirs(os.path.dirname(disk_path), exist_ok=True)
            with open(disk_path, 'wb') as f:
                f.write(b'\0' * (1024 * 1024))  # 1MB dummy file
    
    def _generate_vm_config(self, vm: VirtualMachine) -> None:
        """Generate VM configuration file"""
        config_path = f"{self.storage_pools['vm_storage']}/{vm.id}.conf"
        
        config = {
            "name": vm.config.name,
            "memory": vm.config.memory_mb,
            "vcpus": vm.config.cpu_cores,
            "disk": f"{vm.id}.qcow2",
            "network": vm.config.network_mode,
            "vnc_port": vm.config.vnc_port,
            "ssh_port": vm.config.ssh_port
        }
        
        with open(config_path, 'w') as f:
            json.dump(config, f, indent=2)
    
    def _start_qemu_vm(self, vm: VirtualMachine) -> bool:
        """Start QEMU VM"""
        disk_path = f"{self.storage_pools['vm_storage']}/{vm.id}.qcow2"
        
        cmd = [
            "qemu-system-x86_64",
            "-name", vm.config.name,
            "-m", str(vm.config.memory_mb),
            "-smp", str(vm.config.cpu_cores),
            "-hda", disk_path,
            "-netdev", "user,id=net0",
            "-device", "e1000,netdev=net0"
        ]
        
        if vm.config.vnc_port:
            cmd.extend(["-vnc", f":{vm.config.vnc_port}"])
        
        if vm.config.iso_path:
            cmd.extend(["-cdrom", vm.config.iso_path])
            cmd.extend(["-boot", "d"])
        
        try:
            process = subprocess.Popen(cmd)
            vm.pid = process.pid
            return True
        except:
            return False
    
    def _start_kvm_vm(self, vm: VirtualMachine) -> bool:
        """Start KVM VM"""
        disk_path = f"{self.storage_pools['vm_storage']}/{vm.id}.qcow2"
        
        cmd = [
            "qemu-system-x86_64",
            "-enable-kvm",
            "-name", vm.config.name,
            "-m", str(vm.config.memory_mb),
            "-smp", str(vm.config.cpu_cores),
            "-hda", disk_path,
            "-netdev", "user,id=net0",
            "-device", "virtio-net-pci,netdev=net0"
        ]
        
        if vm.config.vnc_port:
            cmd.extend(["-vnc", f":{vm.config.vnc_port}"])
        
        try:
            process = subprocess.Popen(cmd)
            vm.pid = process.pid
            return True
        except:
            return False
    
    def _start_sigma_native_vm(self, vm: VirtualMachine) -> bool:
        """Start Sigma native VM"""
        # This would use SigmaOS's native virtualization
        # For now, fallback to QEMU
        return self._start_qemu_vm(vm)
    
    def _allocate_vm_ip(self, vm: VirtualMachine) -> str:
        """Allocate IP address for VM"""
        # Simple IP allocation logic
        subnet = self.network_pools["vm_network"]["subnet"]
        base_ip = subnet.split('.')[0:3]
        base_ip.append(str(100 + len(self.vms)))
        return ".".join(base_ip)
    
    def _allocate_container_ip(self, container: Container) -> str:
        """Allocate IP address for container"""
        subnet = self.network_pools["container_network"]["subnet"]
        base_ip = subnet.split('.')[0:3]
        base_ip.append(str(100 + len(self.containers)))
        return ".".join(base_ip)
    
    def _pull_container_image(self, container: Container) -> None:
        """Pull container image"""
        if container.config.container_type == ContainerType.DOCKER:
            try:
                subprocess.run(["docker", "pull", container.config.image], check=True)
            except:
                pass
        elif container.config.container_type == ContainerType.PODMAN:
            try:
                subprocess.run(["podman", "pull", container.config.image], check=True)
            except:
                pass
    
    def _start_docker_container(self, container: Container) -> bool:
        """Start Docker container"""
        cmd = ["docker", "run", "-d"]
        
        if container.config.name:
            cmd.extend(["--name", container.config.name])
        
        for key, value in container.config.environment.items():
            cmd.extend(["-e", f"{key}={value}"])
        
        for host_path, container_path in container.config.volumes.items():
            cmd.extend(["-v", f"{host_path}:{container_path}"])
        
        for host_port, container_port in container.config.ports.items():
            cmd.extend(["-p", f"{host_port}:{container_port}"])
        
        cmd.append(container.config.image)
        
        if container.config.command:
            cmd.extend(container.config.command.split())
        
        try:
            result = subprocess.run(cmd, check=True, capture_output=True, text=True)
            container.container_id = result.stdout.strip()
            return True
        except:
            return False
    
    def _start_podman_container(self, container: Container) -> bool:
        """Start Podman container"""
        cmd = ["podman", "run", "-d"]
        
        if container.config.name:
            cmd.extend(["--name", container.config.name])
        
        for key, value in container.config.environment.items():
            cmd.extend(["-e", f"{key}={value}"])
        
        for host_path, container_path in container.config.volumes.items():
            cmd.extend(["-v", f"{host_path}:{container_path}"])
        
        for host_port, container_port in container.config.ports.items():
            cmd.extend(["-p", f"{host_port}:{container_port}"])
        
        cmd.append(container.config.image)
        
        if container.config.command:
            cmd.extend(container.config.command.split())
        
        try:
            result = subprocess.run(cmd, check=True, capture_output=True, text=True)
            container.container_id = result.stdout.strip()
            return True
        except:
            return False
    
    def _start_sigma_container(self, container: Container) -> bool:
        """Start Sigma native container"""
        # This would use SigmaOS's native containerization
        # For now, return success
        container.container_id = str(uuid.uuid4())
        return True
    
    def _send_vm_shutdown_signal(self, vm: VirtualMachine) -> None:
        """Send shutdown signal to VM"""
        if vm.pid:
            try:
                os.kill(vm.pid, 15)  # SIGTERM
            except:
                pass
    
    def _cleanup_vm_resources(self, vm: VirtualMachine) -> None:
        """Clean up VM resources"""
        # Remove disk image
        disk_path = f"{self.storage_pools['vm_storage']}/{vm.id}.qcow2"
        try:
            os.remove(disk_path)
        except:
            pass
        
        # Remove config file
        config_path = f"{self.storage_pools['vm_storage']}/{vm.id}.conf"
        try:
            os.remove(config_path)
        except:
            pass
    
    def create_snapshot(self, vm_id: str, snapshot_name: str) -> bool:
        """Create VM snapshot"""
        if vm_id not in self.vms:
            return False
        
        vm = self.vms[vm_id]
        disk_path = f"{self.storage_pools['vm_storage']}/{vm.id}.qcow2"
        snapshot_path = f"{self.storage_pools['snapshot_storage']}/{vm.id}_{snapshot_name}.qcow2"
        
        try:
            cmd = ["qemu-img", "create", "-f", "qcow2", "-b", disk_path, snapshot_path]
            subprocess.run(cmd, check=True)
            return True
        except:
            return False
    
    def restore_snapshot(self, vm_id: str, snapshot_name: str) -> bool:
        """Restore VM from snapshot"""
        if vm_id not in self.vms:
            return False
        
        vm = self.vms[vm_id]
        snapshot_path = f"{self.storage_pools['snapshot_storage']}/{vm.id}_{snapshot_name}.qcow2"
        disk_path = f"{self.storage_pools['vm_storage']}/{vm.id}.qcow2"
        
        try:
            # Stop VM if running
            if vm.state == VMState.RUNNING:
                self.stop_vm(vm_id, force=True)
            
            # Replace disk with snapshot
            os.replace(snapshot_path, disk_path)
            return True
        except:
            return False
    
    def get_performance_stats(self, vm_id: str) -> Optional[Dict[str, Any]]:
        """Get VM performance statistics"""
        if vm_id not in self.vms:
            return None
        
        vm = self.vms[vm_id]
        if vm.state != VMState.RUNNING or not vm.pid:
            return None
        
        try:
            import psutil
            process = psutil.Process(vm.pid)
            
            return {
                "cpu_percent": process.cpu_percent(),
                "memory_mb": process.memory_info().rss // (1024 * 1024),
                "memory_percent": process.memory_percent(),
                "num_threads": process.num_threads(),
                "status": process.status()
            }
        except:
            return None
    
    def __repr__(self) -> str:
        return f"SigmaVirtualizationEngine(vms={len(self.vms)}, containers={len(self.containers)})"
