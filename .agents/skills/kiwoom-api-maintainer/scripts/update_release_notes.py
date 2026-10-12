#!/usr/bin/env python3
"""제품 변경 설명을 Unreleased에 중복 없이 반영하고 같은 staging 범위를 검사한다."""
import argparse
import re
import subprocess
from pathlib import Path
import sys

CATEGORIES = ('추가', '변경', '수정', '삭제')
KEY = re.compile(r'[a-z][a-z0-9]*(?:-[a-z0-9]+)*\Z')


def section(text):
    match = re.search(r'^## \[Unreleased\][ \t]*$', text, re.MULTILINE)
    if not match:
        raise ValueError('CHANGELOG.md needs an Unreleased section')
    next_heading = re.search(r'^## ', text[match.end():], re.MULTILINE)
    end = match.end() + next_heading.start() if next_heading else len(text)
    return match.end(), end


def update(text, key, category, description):
    if not KEY.fullmatch(key) or len(key) > 80:
        raise ValueError('Use a stable product-change slug as the note key')
    if category not in CATEGORIES or not description.strip() or '\n' in description or '<!--' in description:
        raise ValueError('Use a supported category and one nonempty product-description line')
    start, end = section(text)
    body = text[start:end]
    marker = f'<!-- change:{key} -->'
    line = f'- {description.strip()} {marker}'
    matches = list(re.finditer(r'^- .*' + re.escape(marker) + r'[ \t]*$', body, re.MULTILINE))
    if len(matches) > 1:
        raise ValueError('Duplicate change keys; reconcile release notes first')
    # 같은 key를 재실행하면 기존 항목만 바꾸고 중복을 만들지 않는다.
    if matches:
        body = body[:matches[0].start()] + body[matches[0].end():]
    heading = f'### {category}'
    target = re.search(r'^' + re.escape(heading) + r'[ \t]*$', body, re.MULTILINE)
    if target:
        pos = target.end()
        body = body[:pos] + '\n\n' + line + '\n\n' + body[pos:].lstrip('\n')
    else:
        body = body.rstrip() + '\n\n' + heading + '\n\n' + line + '\n'
    tail = text[end:]
    return text[:start] + body.rstrip() + ('\n\n' + tail if tail else '\n')


def relevant(path):
    return (path in ('Cargo.toml', 'Cargo.lock', 'docs/api-inventory.json')
            or path.startswith('docs/contracts/')
            or bool(re.match(r'(crates|apps)/[^/]+/(src/|Cargo.toml$)', path)))


def git(repo, *args):
    return subprocess.check_output(['git', '-C', str(repo), *args], text=True)


def check(repo, staged):
    options = ['--cached'] if staged else ['HEAD']
    paths = git(repo, 'diff', *options, '--name-only', '--diff-filter=ACDMRT').splitlines()
    affected = [path for path in paths if relevant(path)]
    untracked = []
    if not staged:
        untracked = git(repo, 'ls-files', '--others', '--exclude-standard').splitlines()
        affected += [path for path in untracked if relevant(path)]
        paths += untracked
    if not affected:
        return
    if 'CHANGELOG.md' not in paths:
        raise ValueError('Product/API changes need CHANGELOG.md in the same selected diff')
    target = git(repo, 'show', ':CHANGELOG.md') if staged else (repo / 'CHANGELOG.md').read_text()
    start, end = section(target)
    body = target[start:end]
    changes = git(repo, 'diff', *options, '--unified=0', '--', 'CHANGELOG.md')
    additions = ([line for line in target.splitlines() if line.startswith('- ')]
                 if 'CHANGELOG.md' in untracked else
                 [line[1:] for line in changes.splitlines() if line.startswith('+- ')])
    if not any(line.strip() in body for line in additions):
        raise ValueError('Selected product/API change needs an added or updated Unreleased entry')
    # 내용의 정확성과 API 영향·증거의 대응은 에이전트가 실제로 읽는다.


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    note = commands.add_parser('note')
    note.add_argument('--repo', type=Path, required=True)
    note.add_argument('--key', required=True)
    note.add_argument('--category', choices=CATEGORIES, required=True)
    note.add_argument('--text', required=True)
    verify = commands.add_parser('check')
    verify.add_argument('--repo', type=Path, required=True)
    verify.add_argument('--staged', action='store_true')
    args = parser.parse_args()
    try:
        if args.command == 'note':
            path = args.repo / 'CHANGELOG.md'
            original = path.read_text()
            modified = update(original, args.key, args.category, args.text)
            if modified != original:
                path.write_text(modified)
        else:
            check(args.repo, args.staged)
        return 0
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(f'Release note check failed: {error}', file=sys.stderr)
        return 2


if __name__ == '__main__':
    sys.exit(main())
