# Developer Certificate of Origin (DCO)

To ensure that the SigmaOS project remains legally clean and that all code can be safely licensed under the MIT License, we use the Developer Certificate of Origin (DCO).

The DCO is a lightweight way for contributors to certify that they wrote or otherwise have the right to submit the code they are contributing to the project.

## The DCO Text

By making a contribution to this project, I certify that:

(a) The contribution was created in whole or in part by me and I have the right to submit it under the open source license indicated in the file; or
(b) The contribution is based upon previous work that, to the best of my knowledge, is covered under an appropriate open source license and I have the right under that license to submit that work with modifications, whether created in whole or in part by me, under the same open source license (unless I am permitted to submit under a different license), as indicated in the file; or
(c) The contribution was provided directly to me by some other person who certified (a), (b) or (c) and I have not modified it.
(d) I understand and agree that this project and the contribution are public and that a record of the contribution (including all personal information I submit with it, including my sign-off) is maintained indefinitely and may be redistributed consistent with this project or the open source license(s) involved.

## How to Sign Your Commits

We require that all commits be signed off. This is simply adding a line to your commit message that looks like:

```text
Signed-off-by: Jane Doe <jane.doe@example.com>
```

You can automatically add this line to your commits by using the `-s` or `--signoff` flag with `git commit`:

```bash
git commit -s -m "feat(network): implement secure mesh routing"
```

If you forgot to sign off your last commit, you can amend it:
```bash
git commit --amend -s
```
