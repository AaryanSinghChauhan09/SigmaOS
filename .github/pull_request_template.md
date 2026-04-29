# SigmaOS Sovereign Pull Request

## Pull Request Description

Please include a summary of the change and which issue is fixed. Include relevant motivation and context.

Fixes # (issue)

## Type of change

- [ ] Bug fix (non-breaking change which fixes an issue)
- [ ] New feature (non-breaking change which adds functionality)
- [ ] Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] Documentation update

## Developer Workflow & Quality Checklist

- [ ] My code follows the SigmaOS C11 / C++11 modular style guidelines
- [ ] I have performed a self-review of my own code
- [ ] I have commented my code, particularly in hard-to-understand areas
- [ ] I have made corresponding changes to the `docs/` and `wiki/` documentation
- [ ] My changes generate no new warnings (verified with `clang-tidy`)
- [ ] I have compiled the module with `SANITIZE=1 DEBUG=1` and found no memory leaks or undefined behavior
- [ ] I have added/updated Unit & Integration tests that prove my fix is effective or that my feature works
- [ ] Any dependent changes have been merged and published in downstream modules
