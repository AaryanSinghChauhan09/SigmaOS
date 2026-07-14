# Community Guide

> **Last Updated**: 2026-07-13

This document provides a comprehensive guide for the SigmaOS community, including how to contribute, participate, and engage with the project.

## Getting Started

### Join the Community

Join the SigmaOS community through various channels:

- **GitHub**: Star and watch the repository for updates
- **Discord**: Join the SigmaOS Discord server for real-time discussion
- **Mailing List**: Subscribe to the development mailing list
- **Twitter**: Follow @SigmaOS for announcements

### First Steps

1. **Read the Documentation**: Start with [README.md](README.md) for an overview
2. **Explore the Architecture**: Read [ARCHITECTURE.md](ARCHITECTURE.md) to understand the system
3. **Check the FAQ**: Review [FAQ.md](FAQ.md) for common questions
4. **Set Up Development**: Follow [INSTALL.md](INSTALL.md) to set up your development environment

## Contributing

### Ways to Contribute

There are many ways to contribute to SigmaOS:

- **Code**: Submit pull requests for features and bug fixes
- **Documentation**: Improve documentation and write guides
- **Testing**: Test on various hardware and report bugs
- **Design**: Provide feedback on architecture and design
- **Translation**: Translate documentation to other languages
- **Community**: Help newcomers and answer questions

### Contribution Workflow

1. **Fork the Repository**: Create your own fork on GitHub
2. **Create a Branch**: Use descriptive branch names (e.g., `feature/my-feature`)
3. **Make Changes**: Implement your changes following coding standards
4. **Add Tests**: Include tests for your changes
5. **Update Documentation**: Update relevant documentation
6. **Submit PR**: Open a pull request with a clear description
7. **Review**: Address feedback from maintainers
8. **Merge**: Your PR will be merged once approved

See [CONTRIBUTING.md](CONTRIBUTING.md) for detailed contribution guidelines.

### Coding Standards

Follow these coding standards:

- **Rust**: Use `cargo fmt` and `cargo clippy`
- **Zig**: Use `zig fmt`
- **Nim**: Use `nimpretty`
- **Documentation**: Document all public APIs
- **Tests**: Include unit tests for new features
- **No External Dependencies**: Implement from first principles where possible

### Pull Request Guidelines

When submitting a pull request:

- **Clear Title**: Use descriptive commit messages
- **Description**: Explain what and why you changed
- **Testing**: Describe how you tested your changes
- **Documentation**: Update relevant documentation
- **Breaking Changes**: Highlight any breaking changes

## Communication

### Discord Channels

Join our Discord server for real-time discussion:

- **#general**: General discussion and questions
- **#development**: Development discussion
- **#help**: Get help with issues
- **#announcements**: Project announcements
- **#off-topic**: Casual conversation

### Mailing List

Subscribe to the development mailing list:

- **sigmaos-dev@lists.sigmaos.org**: Development discussion
- **sigmaos-announce@lists.sigmaos.org**: Announcements only

### GitHub Discussions

Use GitHub Discussions for:

- **Feature Requests**: Propose new features
- **Questions**: Ask questions about the project
- **Showcase**: Show what you've built with SigmaOS
- **Feedback**: Provide feedback on the project

### Code of Conduct

All community members must follow the [Code of Conduct](CODE_OF_CONDUCT.md). Key principles:

- **Respect**: Be respectful and inclusive
- **Constructive**: Focus on constructive feedback
- **Welcome**: Welcome newcomers and help them learn
- **Privacy**: Respect privacy and confidentiality

## Package Recipes

### Creating Package Recipes

SigmaOS uses a SlackBuilds-style package recipe system:

1. **Copy Template**: Copy `example.hello.sigma.recipe` to `your-package.sigma.recipe`
2. **Set Curation**: Set `curation=community` for community packages
3. **Add Hash**: Set a real `hash=sha256:...` for verification
4. **Submit PR**: Open a pull request with your recipe
5. **CI Check**: CI runs `./scripts/ci_branch_check.sh`
6. **Merge**: After merge, wiki sync publishes package notes

### Official vs Community Recipes

- **Official Recipes**: Use `curation=official` and require maintainer signature
- **Community Recipes**: Use `curation=community` and are community-maintained

### Recipe Format

Package recipes follow this format:

```toml
[package]
name = "example-package"
version = "1.0.0"
curation = "community"
hash = "sha256:..."

[build]
source = "https://example.com/package.tar.gz"
build_system = "cargo"
dependencies = []

[install]
files = [
    "bin/example",
    "lib/example.so",
]
```

## Events

### Community Events

Participate in community events:

- **Monthly Meetings**: Monthly community meetings on Discord
- **Hackathons**: Periodic hackathons for feature development
- **Workshops**: Educational workshops on SigmaOS development
- **Conferences**: Presentations at open source conferences

### Organizing Events

Want to organize an event?

1. **Proposal**: Propose the event to the community
2. **Planning**: Plan the event with community input
3. **Promotion**: Promote the event through community channels
4. **Execution**: Execute the event with community support
5. **Follow-up**: Share results and feedback

## Recognition

### Contributor Recognition

Contributors are recognized through:

- **Contributors List**: Listed in CONTRIBUTORS.md
- **Release Notes**: Mentioned in release notes
- **Blog Posts**: Featured in blog posts
- **Awards**: Annual community awards

### Becoming a Maintainer

Experienced contributors can become maintainers:

1. **Consistent Contributions**: Regular contributions over time
2. **Code Quality**: High-quality code and reviews
3. **Community Engagement**: Active community participation
4. **Mentorship**: Help mentor new contributors
5. **Nomination**: Be nominated by existing maintainers

## Support

### Getting Help

Get help through these channels:

- **GitHub Issues**: Report bugs and ask questions
- **Discord**: Join #help channel for real-time help
- **Mailing List**: Ask questions on the mailing list
- **Documentation**: Read the documentation first

### Giving Help

Help others by:

- **Answering Questions**: Answer questions on Discord and mailing list
- **Reviewing PRs**: Review pull requests from others
- **Mentoring**: Mentor new contributors
- **Writing Documentation**: Write guides and tutorials

### Support Policy

See [SUPPORT.md](SUPPORT.md) for the support policy.

## Governance

### Project Governance

SigmaOS is governed by:

- **BDFL**: Benevolent Dictator For Life makes final decisions
- **Maintainers**: Experienced contributors with merge rights
- **Contributors**: Community members who contribute
- **Community**: All users and interested parties

### Decision Making

Decisions are made through:

- **Consensus**: Community consensus for major decisions
- **Maintainer Vote**: Maintainer vote for technical decisions
- **BDFL Decision**: BDFL makes final decision when needed

### Governance Changes

Propose governance changes by:

1. **Discussion**: Discuss the change in the community
2. **Proposal**: Create a formal proposal
3. **Vote**: Community vote on the proposal
4. **Implementation**: Implement approved changes

## Resources

### Documentation

- [README.md](README.md): Project overview
- [ARCHITECTURE.md](ARCHITECTURE.md): System architecture
- [FAQ.md](FAQ.md): Frequently asked questions
- [INSTALL.md](INSTALL.md): Installation guide
- [CONTRIBUTING.md](CONTRIBUTING.md): Contribution guidelines
- [SECURITY_POLICY.md](SECURITY_POLICY.md): Security policy
- [SUPPORT.md](SUPPORT.md): Support resources

### Development Resources

- [GitHub Repository](https://github.com/AaryanSinghChauhan09/SigmaOS): Source code
- [Issue Tracker](https://github.com/AaryanSinghChauhan09/SigmaOS/issues): Bug tracking
- [Pull Requests](https://github.com/AaryanSinghChauhan09/SigmaOS/pulls): Pull requests
- [Actions](https://github.com/AaryanSinghChauhan09/SigmaOS/actions): CI/CD

### Learning Resources

- [Rust Book](https://doc.rust-lang.org/book/): Learn Rust
- [Zig Guide](https://ziglearn.org/): Learn Zig
- [Nim Tutorial](https://nim-lang.org/docs/tutorial.html): Learn Nim
- [Capability-Based Security](https://cap-lore.com/CapTheory/): Learn capabilities

## Contact

### Project Contact

- **Email**: contact@sigmaos.org
- **Discord**: Join the SigmaOS Discord server
- **Twitter**: @SigmaOS
- **GitHub**: https://github.com/AaryanSinghChauhan09/SigmaOS

### Security Contact

- **Email**: security@sigmaos.org
- **PGP Key**: Available on GitHub

See [SECURITY_POLICY.md](SECURITY_POLICY.md) for security reporting.

## Acknowledgments

Thank you to all contributors who have helped make SigmaOS possible. Your contributions are greatly appreciated.

---

*Last Updated: 2026-07-13*
