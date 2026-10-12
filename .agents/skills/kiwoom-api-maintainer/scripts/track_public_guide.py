#!/usr/bin/env python3
"""공개 가이드의 메뉴와 계약 표만 비공개 관찰 자료로 수집한다."""
import argparse
import datetime as dt
import hashlib
from html.parser import HTMLParser
import json
from pathlib import Path
import re
import sys
import time
import urllib.parse
import urllib.request

MENU_URL = 'https://openapi.kiwoom.com/m/guide/apiguide?dummyVal=0'
GUIDE_ROOT = 'https://openapi.kiwoom.com/m/guide/apiguide'
CALL = re.compile(r"goToGuide\(\s*['\"]([A-Za-z0-9]+)['\"]\s*,\s*['\"]([A-Za-z0-9]+)['\"]\s*\)")
MAX_BYTES = 8 * 1024 * 1024
FORMAT = 1


def public_url(url):
    """공개 문서 경로 밖으로 요청·리다이렉트하지 않는다."""
    parsed = urllib.parse.urlsplit(url)
    allowed_path = re.fullmatch(r'/m/guide/apiguide(?:/[A-Za-z0-9]+/[A-Za-z0-9]+)?', parsed.path)
    if (parsed.scheme != 'https' or parsed.netloc != 'openapi.kiwoom.com'
            or not allowed_path or parsed.fragment
            or parsed.query not in ('', 'dummyVal=0')):
        raise ValueError('Only public mobile guide URLs are allowed')
    return url


class GuideRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        public_url(newurl)
        return super().redirect_request(req, fp, code, msg, headers, newurl)


def fetch(url):
    request = urllib.request.Request(public_url(url), headers={'User-Agent': 'IndependentGuideObserver/1.0'})
    with urllib.request.build_opener(GuideRedirect()).open(request, timeout=20) as response:
        public_url(response.url)
        data = response.read(MAX_BYTES + 1)
        if len(data) > MAX_BYTES:
            raise ValueError('Public guide exceeds response bound')
        return data.decode('utf-8')


def menu_routes(html):
    routes = {}
    for group, api_id in CALL.findall(html):
        if api_id in routes and routes[api_id] != group:
            raise ValueError(f'Conflicting guide routes for {api_id}')
        routes[api_id] = group
    if not routes:
        raise ValueError('No public API IDs extracted; review the parser')
    return dict(sorted(routes.items()))


def normalized(text):
    return ' '.join(text.split())


class Tables(HTMLParser):
    """표의 식별과 셀 경계를 보존하고 HTML·코드 예제는 저장하지 않는다."""
    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.tables = []
        self.table = None
        self.row = None
        self.cell = None
        self.caption = None
        self.ignored = 0
        self.active_ids = set()
        self.li_depth = 0
        self.active_depth = None
        self.active_item_ids = set()
        self.active_span = None

    def handle_starttag(self, tag, attrs):
        attrs = dict(attrs)
        if tag == 'li':
            self.li_depth += 1
            if 'active' in attrs.get('class', '').split():
                self.active_depth = self.li_depth
                self.active_item_ids = {api_id for _, api_id in CALL.findall(attrs.get('onclick', ''))}
        if tag == 'span' and self.active_depth is not None:
            self.active_span = []
        if tag in ('script', 'style', 'pre', 'code'):
            self.ignored += 1
        if self.ignored:
            return
        if tag == 'table':
            if self.table is not None:
                raise ValueError('Nested contract tables need parser review')
            self.table = {'caption': '', 'rows': []}
        elif self.table is not None:
            if tag == 'caption':
                self.caption = []
            elif tag == 'tr':
                self.row = []
            elif tag in ('td', 'th'):
                self.cell = {'text': [], 'kind': tag,
                             'rowspan': attrs.get('rowspan', '1'),
                             'colspan': attrs.get('colspan', '1')}
            elif tag == 'br':
                self.handle_data(' ')

    def handle_data(self, data):
        if self.ignored:
            return
        if self.active_span is not None:
            self.active_span.append(data)
        if self.caption is not None:
            self.caption.append(data)
        if self.cell is not None:
            self.cell['text'].append(data)

    def handle_endtag(self, tag):
        if tag == 'span' and self.active_span is not None:
            visible_id = normalized(''.join(self.active_span))
            if visible_id in self.active_item_ids:
                self.active_ids.add(visible_id)
            self.active_span = None
        if tag == 'li':
            if self.li_depth == self.active_depth:
                self.active_depth = None
                self.active_item_ids = set()
                self.active_span = None
            self.li_depth = max(0, self.li_depth - 1)
        if tag in ('script', 'style', 'pre', 'code'):
            self.ignored = max(0, self.ignored - 1)
            return
        if self.ignored or self.table is None:
            return
        if tag == 'caption' and self.caption is not None:
            self.table['caption'] = normalized(''.join(self.caption))
            self.caption = None
        elif tag in ('td', 'th') and self.cell is not None:
            if self.row is None:
                raise ValueError('Contract cell without row')
            self.cell['text'] = normalized(''.join(self.cell['text']))
            self.row.append(self.cell)
            self.cell = None
        elif tag == 'tr' and self.row is not None:
            self.table['rows'].append(self.row)
            self.row = None
        elif tag == 'table':
            self.tables.append(self.table)
            self.table = None


def contract_tables(html, api_id):
    parser = Tables()
    parser.feed(html)
    parser.close()
    if api_id not in parser.active_ids:
        raise ValueError(f'Selected guide ID is not {api_id}')
    tables = [table for table in parser.tables if table['caption']]
    labels = {row[0]['text'] for table in tables for row in table['rows'] if row}
    if not {'Method', 'URL'}.issubset(labels):
        raise ValueError('Missing basic Method/URL contract table')
    captions = ' '.join(table['caption'] for table in tables)
    if not all(side in captions for side in ('요청', '응답')):
        raise ValueError('Missing request/response contract tables')
    return tables


def digest(value):
    data = json.dumps(value, ensure_ascii=False, sort_keys=True, separators=(',', ':')).encode()
    return hashlib.sha256(data).hexdigest()


def parser_fingerprint():
    return hashlib.sha256(Path(__file__).read_bytes()).hexdigest()


def capture(args):
    if bool(args.menu_file) != bool(args.detail_dir):
        raise ValueError('Offline mode requires both --menu-file and --detail-dir')
    if args.out.exists():
        raise ValueError('Choose a new snapshot directory; never overwrite a baseline')
    menu = args.menu_file.read_text() if args.menu_file else fetch(MENU_URL)
    routes = menu_routes(menu)
    requested = sorted(set(args.ids.split(','))) if args.ids else sorted(routes)
    if not requested or any(api_id not in routes for api_id in requested):
        raise ValueError('Requested IDs must occur in the fresh guide menu')
    inventory = json.loads(args.inventory.read_text())
    local_routes = {entry['id']: entry['public_guide_group'] for entry in inventory['entries']}
    if len(local_routes) != len(inventory['entries']):
        raise ValueError('Duplicate IDs in local inventory')
    snapshot = {'format': FORMAT, 'parser': parser_fingerprint(),
                'observed_at': dt.datetime.now(dt.timezone.utc).isoformat(),
                'source': MENU_URL, 'acquisition': 'offline_fixture' if args.menu_file else 'public_https',
                'scope': 'all_listed' if not args.ids else 'subset',
                'routes': routes, 'requested_ids': requested, 'pages': {}, 'errors': {},
                'inventory_difference': {
                    'added': sorted(routes.keys() - local_routes.keys()),
                    'removed': sorted(local_routes.keys() - routes.keys()),
                    'rerouted': sorted(api_id for api_id in routes.keys() & local_routes.keys()
                                       if routes[api_id] != local_routes[api_id])}}
    args.out.mkdir(parents=True)
    for api_id in requested:
        url = public_url(f'{GUIDE_ROOT}/{routes[api_id]}/{api_id}')
        try:
            if args.detail_dir:
                html = (args.detail_dir / f'{api_id}.html').read_text()
            else:
                time.sleep(args.interval)
                html = fetch(url)
            tables = contract_tables(html, api_id)
            snapshot['pages'][api_id] = {'source': url, 'sha256': digest(tables), 'tables': tables}
        except (OSError, ValueError, UnicodeError) as error:
            snapshot['errors'][api_id] = str(error)
    (args.out / 'snapshot.json').write_text(json.dumps(snapshot, ensure_ascii=False, indent=2) + '\n')
    return 2 if snapshot['errors'] else 0


def validate_snapshot(snapshot):
    if snapshot.get('format') != FORMAT or not snapshot.get('parser'):
        raise ValueError('Unsupported snapshot format')
    routes = snapshot['routes']
    requested = snapshot['requested_ids']
    if not routes or not requested or len(requested) != len(set(requested)):
        raise ValueError('Empty or duplicate snapshot scope')
    if not set(requested).issubset(routes):
        raise ValueError('Requested IDs outside snapshot menu')
    if snapshot['scope'] not in ('all_listed', 'subset'):
        raise ValueError('Unknown snapshot scope')
    if snapshot['scope'] == 'all_listed' and set(requested) != set(routes):
        raise ValueError('Whole-catalog scope omits menu IDs')
    if set(snapshot['pages']) | set(snapshot['errors']) != set(requested):
        raise ValueError('Snapshot page coverage is inconsistent')
    if set(snapshot['pages']) & set(snapshot['errors']):
        raise ValueError('Successful and failed page sets overlap')
    for api_id, page in snapshot['pages'].items():
        if page['sha256'] != digest(page['tables']):
            raise ValueError(f'Table fingerprint mismatch for {api_id}')
        if page['source'] != public_url(f'{GUIDE_ROOT}/{routes[api_id]}/{api_id}'):
            raise ValueError('Snapshot source/route mismatch')


def compare(before, after):
    validate_snapshot(before)
    validate_snapshot(after)
    old, new = set(before['routes']), set(after['routes'])
    reasons = []
    if before['parser'] != after['parser']:
        reasons.append('parser_changed')
    if before['scope'] != after['scope'] or (before['scope'] == 'subset'
            and set(before['requested_ids']) != set(after['requested_ids'])):
        reasons.append('scope_changed')
    if before['errors'] or after['errors']:
        reasons.append('failed_pages')
    common_pages = before['pages'].keys() & after['pages'].keys()
    changed = sorted(api_id for api_id in common_pages
                     if before['pages'][api_id]['sha256'] != after['pages'][api_id]['sha256'])
    result = {'before_observed_at': before['observed_at'], 'after_observed_at': after['observed_at'],
              'coverage': {'before_scope': before['scope'], 'after_scope': after['scope'],
                           'before_failed': before['errors'], 'after_failed': after['errors'],
                           'incomparable_reasons': reasons,
                           'not_table_compared': sorted((old | new) - set(common_pages)),
                           'excluded': ['prose_outside_tables', 'samples', 'notices', 'live_broker_behavior']},
              'added': sorted(new - old), 'removed_candidates': sorted(old - new),
              'rerouted': sorted(api_id for api_id in old & new if before['routes'][api_id] != after['routes'][api_id]),
              'table_changed': changed,
              'table_diffs': {api_id: {'before': before['pages'][api_id]['tables'],
                                      'after': after['pages'][api_id]['tables']} for api_id in changed}}
    result['status'] = ('incomplete' if reasons else 'changed' if any(result[key] for key in
                        ('added', 'removed_candidates', 'rerouted', 'table_changed')) else 'unchanged_extracted_scope')
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    commands = parser.add_subparsers(dest='command', required=True)
    cap = commands.add_parser('capture')
    cap.add_argument('--inventory', type=Path, required=True)
    cap.add_argument('--out', type=Path, required=True)
    cap.add_argument('--ids')
    cap.add_argument('--menu-file', type=Path)
    cap.add_argument('--detail-dir', type=Path)
    cap.add_argument('--interval', type=float, default=0.5)
    diff = commands.add_parser('diff')
    diff.add_argument('--before', type=Path, required=True)
    diff.add_argument('--after', type=Path, required=True)
    diff.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    try:
        if args.command == 'capture':
            if args.interval < 0.5:
                raise ValueError('Minimum request interval is 0.5 seconds')
            return capture(args)
        if args.out.exists():
            raise ValueError('Choose a new diff path')
        result = compare(json.loads(args.before.read_text()), json.loads(args.after.read_text()))
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
        return {'incomplete': 2, 'changed': 1, 'unchanged_extracted_scope': 0}[result['status']]
    except (OSError, ValueError, KeyError, TypeError) as error:
        print(f'Observation failed: {error}', file=sys.stderr)
        return 2


if __name__ == '__main__':
    sys.exit(main())
