# Pull Request Template

## Description
<!-- Describe your changes in detail -->

## Related Issues
<!-- Link to related issues or pull requests -->

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update
- [ ] Performance improvement
- [ ] Security fix

## Testing
<!-- Describe how you tested your changes -->

## Checklist
- [ ] I have read the AGENTS.md and followed the coding conventions
- [ ] I have tested the changes using standalone file compilation
- [ ] I have updated the relevant documentation
- [ ] I have added tests for new functionality
- [ ] My changes follow the zero-dependency philosophy
- [ ] My changes maintain the capability-based security model
- [ ] I have run the security scanning and fixed any issues

## Architecture Compliance
- [ ] Zero-Dependency: No new std dependencies in kernel code
- [ ] Capability-Based Security: Uses capability tokens for authorization
- [ ] WDM Driver Model: Follows Windows NT driver abstractions
- [ ] Memory Management: Respects Paged/NonPaged memory boundaries

## Performance Impact
<!-- Describe any performance impact of your changes -->

## Backwards Compatibility
<!-- Describe any backwards compatibility concerns -->

## Additional Notes
<!-- Any additional information that might be helpful -->
