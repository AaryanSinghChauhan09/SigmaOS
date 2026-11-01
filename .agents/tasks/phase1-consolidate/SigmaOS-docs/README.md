# SigmaOS-docs

Technical documentation repository for SigmaOS, containing architecture guides, component registry, governance policies, developer cookbook, and project roadmap.

## Structure

```
docs/
├── architecture/          # System design and principles
│   └── adr/              # Architecture Decision Records
├── components/           # Component registry and matrix
├── capabilities/         # Feature status and capabilities
├── subsystems/           # Deep-dive documentation for major subsystems
├── governance/           # Rules, policies, and contribution guidelines
├── cookbook/             # Developer how-to guides
├── roadmap/              # Project milestones and vision
└── assets/               # Images and static assets
```

## Key Documents

- **[Component Matrix](docs/components/COMPONENTS.md)** – Registry of all components (auto-generated)
- **[Governance Policies](docs/governance/)** – Rules and standards for all subsystems
- **[Architecture Overview](docs/architecture/)** – System design and patterns
- **[Roadmap](docs/roadmap/)** – Project milestones and vision

## Auto-Generation

This repository uses GitHub Actions to automatically:

1. **sync-components.yml** – Syncs COMPONENTS.md from the main SigmaOS repo on each push
2. **validate-links.yml** – Validates internal documentation links
3. **publish-pages.yml** – Publishes documentation to GitHub Pages

## Contributing

See [Contribution Guidelines](docs/governance/README.md) for coding standards, process rules, and review expectations.

## License

This documentation is part of the SigmaOS project. See LICENSE file for details.
