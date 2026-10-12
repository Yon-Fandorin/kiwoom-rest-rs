# Release notes in the same change

Use the root Korean `CHANGELOG.md` for user-visible product changes. Describe
observable behavior, contract additions/changes/removals, migration needs and
remaining support limits; do not record task chronology, reviewer identities,
private branches, tool logs, or unexecuted release claims. Keep pending changes
under `[Unreleased]`; move them to a version only as part of an authorized release
with actual version/tag evidence. A package version alone is not a release.

For each authorized API or general feature update, add or revise a concise Korean
entry before staging. Stable product-change keys make reruns update the same
entry instead of duplicating it. Keys are metadata, not dates or commit history.
Substitute the resolved project skill and repository paths:

```sh
python3 "$SKILL/scripts/update_release_notes.py" note --repo "$SDK" \
  --key bounded-query-output --category 변경 \
  --text '조회 도구의 결과 크기를 제한하고 다음 페이지 커서를 반환합니다.'
python3 "$SKILL/scripts/update_release_notes.py" check --repo "$SDK"
```

`note` changes only the matching keyed entry in `Unreleased`; it does not inspect
code or invent an accurate description. Read the final code and entry together.
For a removed upstream API, include the bounded public support impact and
migration/deprecation behavior once independently confirmed; do not describe a
failed observation as confirmed removal. If no product/API behavior changed, no
new release entry is needed solely for a check run or private report.

When a code commit is authorized, stage only the requested code/contracts/tests,
inventory evidence, and their release note; include ordinary feature updates too.
Before that commit, run:

```sh
python3 "$SKILL/scripts/update_release_notes.py" check --repo "$SDK" --staged
```

The checker requires an added or updated Unreleased entry in the **same selected
diff** as staged production Rust, app, Cargo or contract/inventory changes. It
also notices unstaged/new production files in working-tree mode. It is a bounded
path check, not semantic evidence, and does not automatically stage, commit,
push, publish a GitHub release, bump versions, or create tags. The agent must
also assess features implemented outside these known paths and ensure their notes
are included. Tests-only, mechanical edits, and unchanged behavior need human
judgment; if the path check rejects such an edit, explain and review that scope
rather than writing a false product change. No automatic bypass is provided.

API-source observations alone may produce a private report with no commit. For
implemented additions, modifications or removals, check typed/mock evidence and
support limitations before writing the entry. Contract implementation does not
authorize financial execution. Neither note generation nor commit approval
expands authorization to push or release publication.
