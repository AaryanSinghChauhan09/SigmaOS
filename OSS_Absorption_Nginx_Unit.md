# OSS Absorption: NGINX Unit — Application Server

> **Status**: 📋 Planned | **Source Project**: NGINX Unit | **Target Shard**: `SigmaOS Application Runtime Shard`

---

## 1. Executive Summary

NGINX Unit is a lightweight, dynamic web application server that runs code in multiple languages (Python, Go, Node.js, PHP) simultaneously, featuring a fully declarative REST API configuration model that requires no configuration reloads.

SigmaOS absorbs NGINX Unit's **dynamic application process execution** and **zero-reload configuration API**, using them to power `sigma-runtime` for sandboxed web services.

---

## 2. Key Features Absorbed

### 2.1 Dynamic App Lifecycle Manager

Instead of configuring independent WSGI, Node, or Go process managers, services in SigmaOS are defined as declarative JSON configurations loaded into `sigma-runtime`.

```json
{
  "listeners": {
    "*:8080": {
      "pass": "applications/python_app"
    }
  },
  "applications": {
    "python_app": {
      "type": "python 3",
      "processes": 5,
      "path": "/var/www/app/"
    }
  }
}
```

Updating this routing policy is instantaneous and requires no service downtime.

---

## 3. References & Standards

- NGINX Unit — `unit.nginx.org` (Apache-2.0 License)
