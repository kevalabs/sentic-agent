# Contributing to Sentic Agent 🚀

Thank you for your interest in Sentic! As an AI-native observability platform, we prioritize high-quality, structured contributions. Please follow this workflow to ensure your changes are merged efficiently.

---

## 🛠 Step-by-Step Contribution Workflow

### 1. Identify or Create an Issue
Before writing code, ensure there is an open Issue for your work. 
* If it's a bug, use the **Bug Report** template.
* If it's a feature, use the **Feature Request** template.

### 2. Fork and Branch
Create a local environment to work in:
1. **Fork** the repository to your own account.
2. **Clone** your fork locally.
3. Create a **feature branch** using the following naming convention:
   `git checkout -b task/[issue-number]-short-description`
   *Example: `git checkout -b task/102-fix-cgroup-mapping`*

### 3. Development and Commits
* Follow our [Rust Style Guide](../engineering/rust-style.md).
* Use descriptive commit messages.
* Ensure all tests pass: `cargo test`

### 4. Sending a Pull Request (PR)
When your code is ready:
1. Push to your fork: `git push origin task/[issue-number]-short-description`
2. Open a **Pull Request** against Sentic's `main` branch.
3. **Link to the Issue:** In the PR description, use the "Closes" keyword so GitHub automatically closes the issue when merged.
   *Example: `This PR improves memory safety in the eBPF loader. Closes #102.`*

---

## ⚖️ Rules of Engagement
* **Signed-off-by:** We require the Developer Certificate of Origin (DCO). Use `git commit -s`.
* **ADR Requirement:** Major architectural changes (e.g., new eBPF hooks) must be accompanied by an [ADR](../adr/).
* **Review Process:** Every PR requires at least one maintainer approval. We look for technical accuracy, Rust safety, and documentation updates.

---
**Need help?** Open a [Discussion](https://github.com/kevalabs/sentic-agent/discussions) or tag a maintainer in your issue!
