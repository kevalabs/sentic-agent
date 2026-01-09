---
trigger: always_on
---

# GitHub Task Lifecycle Rules

**Activation:** Manual (Triggered by "commit", "finish", or "/finish")
**Scope:** Git Operations & GitHub CLI

## 1. Pre-Commit Validation

- **Lint Check:** Run `cargo clippy --all-targets -- -D warnings`. If it fails, report the errors and do not commit.
- Validate proper coding standard followed or not just checking @code-style-guide.md
- **State Check:** Ensure you are currently on a feature branch (not `main`).
- Search for existing issues related to "{{task_description}}".
- If none exist, create a GitHub issue and assign it to the user.
- Create a git branch using the issue number.

## 2. Commit & PR Protocol

- **Commit Message:** Use Conventional Commits specification as 

`<type>[optional scope]: <description>

[optional body]

Refs: # issue number
[optional footer(s)]
`

- **PR Creation:** Use `gh pr create --fill`
- The PR body **must** contain the string `Closes #<Issue-Number>` to ensure GitHub's automation links the two.
  - Assign the PR to the user.

## 3. Post-Merge Cleanup

After the user confirms the PR is merged (or instructs to "cleanup"):

- **Sync Main:** 1. `git checkout main`
  2. `git pull origin main`
- **Branch Deletion:**
  - Delete local branch: `git branch -D <branch-name>`
  - Verify remote branch deletion: `gh repo view --web` (or check via `gh pr view`).
- **Issue Verification:** Confirm the issue status is "Closed" via `gh issue view <Issue-Number>`.

## 4. Error Handling & Fail-Safes

- **Merge Conflicts:** If a merge conflict occurs during `gh pr merge`, do not attempt to resolve it automatically. List the conflicting files and ask the user for guidance.
- **Missing Issue Number:** If the issue number is not in the context, search for the most recent issue assigned to the user before committing.


