# OSS Absorption: Ansible — Configuration Management

> **Status**: 📋 Planned | **Source Project**: Ansible (Red Hat) | **Target Shard**: `SigmaOS Fleet Configuration Automation`

---

## 1. Executive Summary

Ansible is an agentless IT automation tool that uses SSH and YAML-based "playbooks" to configure systems, deploy applications, and manage infrastructure at scale.

SigmaOS absorbs Ansible's **agentless fleet management model** and **idempotent task execution**, embedding them into `sigma-fleet`, which manages multi-node SigmaOS deployments via native WireGuard tunnels instead of SSH.

---

## 2. Key Features Absorbed

### 2.1 Agentless Fleet Execution (`sigma-fleet`)

From a single control node, an operator pushes declarative YAML playbooks to all cluster nodes simultaneously, over WireGuard-encrypted management channels.

```yaml
# playbooks/deploy_web.yaml
- name: Deploy web frontend
  hosts: web_nodes
  tasks:
    - name: Install sigma package
      sigma.pkg:
        name: frontend-server
        state: present

    - name: Ensure service running
      sigma.service:
        name: frontend-server
        state: started
        enabled: true
```

---

## 3. References & Standards

- Ansible — `ansible.com` (GPL-3.0)
