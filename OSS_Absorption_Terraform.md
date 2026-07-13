# OSS Absorption: Terraform — Infrastructure as Code

> **Status**: 📋 Planned | **Source Project**: HashiCorp Terraform / OpenTofu | **Target Shard**: `SigmaOS Declarative System Provisioning`

---

## 1. Executive Summary

Terraform is an infrastructure-as-code (IaC) tool that allows users to define cloud infrastructure in human-readable HCL configuration files and provision it through a plan/apply workflow.

SigmaOS absorbs the **plan-then-apply declarative workflow** and **state file model** of Terraform/OpenTofu, applying them natively to local system provisioning. Instead of running ad-hoc shell commands to configure a fresh machine, users define the desired system state in HCL, and `sigma-provision` computes and applies the minimal change set.

---

## 2. Key Features Absorbed

### 2.1 Plan/Apply for System Configuration

```hcl
# /etc/sigma/provision.hcl
resource "sigma_user" "dev" {
  name   = "developer"
  groups = ["sudo", "network"]
}

resource "sigma_package" "dev_tools" {
  packages = ["rust", "python3", "git", "helix"]
}

resource "sigma_service" "sshd" {
  enabled = true
  running = true
}
```

```bash
$ sigma provision plan
Σ [PROVISION] Changes to apply:
  + Create user 'developer' (groups: sudo, network)
  + Install 4 packages
  + Enable sshd service

$ sigma provision apply
Σ [PROVISION] Applied. System state saved.
```

---

## 3. References & Standards

- Terraform — `terraform.io` (BSL-1.1 with MPL-2.0 history)
- OpenTofu — `opentofu.org` (MPL-2.0, open-source fork)
