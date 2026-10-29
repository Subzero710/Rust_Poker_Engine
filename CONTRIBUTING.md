# Contributing to Rust Poker Engine

## Select work

Every meaningful change starts from a GitHub Issue.

Before starting work:

1. Select an existing Issue or create one with a clear and bounded scope.
2. Ensure the Issue contains enough information to start the work.
3. For bug reports, include reproduction steps and the expected and actual behavior.
4. Use appropriate labels and milestones when relevant.
5. Move the Issue to `Ready` only when its scope is clear.

## Start work

Update local `main` before starting a change.

Never develop directly on `main`.

Create one branch per Issue from the updated `main` branch.

Use short, descriptive, lowercase branch names:

- `feature/...` for new functionality;
- `fix/...` for bug fixes;
- `docs/...` for documentation changes;
- `experiment/...` only for genuinely isolated experiments.

Move the Issue to `In Progress` when work starts.

## Commits

Keep commits small, focused, and descriptive. Each commit must represent one logical change.

Commit messages are written in English using an imperative action followed by a concise description.

Examples:

```text
Configure local Rust quality automation
Document contribution workflow
```

Do not use vague messages such as `update`, `fix`, `changes`, or `final`.

The project does not use commit-type prefixes such as `feat:`, `fix:`, or `chore:`.

## Local validation

Ensure `pre-commit` is installed, then install the repository hook:

```bash
pre-commit install
```

Before opening or updating a Pull Request, run:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
pre-commit run --all-files
```

All required local checks must pass before the change is ready for review.

## Pull Requests

Push the Issue branch and open a Pull Request into `main`.

The Pull Request must:

- clearly state what changed;
- explain why the change was needed;
- include relevant implementation details;
- reference the corresponding Issue;
- use a closing keyword such as `Closes #<issue>` when the Pull Request resolves the Issue.

Move the Issue to `In Review` when the Pull Request is opened.

Corrections requested during review must continue on the same branch and Pull Request.

Changes reach `main` only through Pull Requests.

Merge only when the Pull Request is ready and every required check passes. Use Squash & Merge so the completed Issue becomes one logical change on `main`.

After the merge:

1. close the Issue if it was not closed automatically;
2. move the Issue to `Done`;
3. delete the temporary branch.