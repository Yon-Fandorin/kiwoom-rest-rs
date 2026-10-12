"""합성 공개 표와 임시 Git 저장소로 변경 감지·릴리즈 동작을 검증한다."""
import argparse
import copy
import json
from pathlib import Path
import subprocess
import tempfile
import unittest
from unittest.mock import patch

import track_public_guide as guide
import update_release_notes as release


def html(api_id='ka10001', required='N', extra=''):
    return f'''<li class="active" onclick="goToGuide('01', '{api_id}')"><span>{api_id}</span></li>
    <script>secret_example = "not-observed"</script>
    <table><caption>기본 정보</caption><tr><th>Method</th><td>POST</td></tr>
    <tr><th>URL</th><td>/synthetic/query</td></tr></table>
    <table><caption>요청 Body 정보 표</caption><tr><th>Element</th><th>Required</th></tr>
    <tr><td>synthetic_field</td><td>{required}</td></tr></table>
    <table><caption>응답 Body 정보 표</caption><tr><td>raw_value</td><td>String</td></tr>{extra}</table>
    <pre>official_sample_must_not_be_saved</pre>'''


def snapshot(routes=None, scope='all_listed'):
    routes = routes or {'ka10001': '01'}
    tables = guide.contract_tables(html(), 'ka10001')
    return {
        'format': 1,
        'parser': 'synthetic-parser',
        'observed_at': 'synthetic-time',
        'scope': scope,
        'routes': routes,
        'requested_ids': ['ka10001'],
        'errors': {},
        'pages': {
            'ka10001': {
                'source': guide.GUIDE_ROOT + '/01/ka10001',
                'tables': tables,
                'sha256': guide.digest(tables),
            },
        },
    }


class GuideTests(unittest.TestCase):
    def test_public_url_rejects_broker_and_credentials(self):
        for url in ('https://api.kiwoom.com/api/dostk/stkinfo', 'https://mockapi.kiwoom.com',
                    'https://user@openapi.kiwoom.com/m/guide/apiguide',
                    'https://openapi.kiwoom.com/official.json',
                    guide.MENU_URL + '&token=synthetic', 'http://openapi.kiwoom.com/m/guide/apiguide'):
            with self.assertRaises(ValueError):
                guide.public_url(url)

    def test_redirect_gate_rejects_external_target(self):
        with self.assertRaises(ValueError):
            guide.GuideRedirect().redirect_request(None, None, 302, '', {}, 'https://api.kiwoom.com')

    def test_menu_case_sensitive_and_nonempty(self):
        routes = guide.menu_routes("goToGuide('15','0B');goToGuide('15','0b')")
        self.assertEqual(routes, {'0B': '15', '0b': '15'})
        with self.assertRaises(ValueError):
            guide.menu_routes('<html>blocked</html>')
        with self.assertRaises(ValueError):
            guide.menu_routes("goToGuide('01','ka1');goToGuide('02','ka1')")

    def test_tables_omit_samples_preserve_required_and_cell_boundaries(self):
        tables = guide.contract_tables(html(extra='<tr><td rowspan="2">signed<br>raw</td></tr>'), 'ka10001')
        encoded = json.dumps(tables)
        self.assertNotIn('official_sample', encoded)
        self.assertNotIn('secret_example', encoded)
        self.assertEqual(tables[-1]['rows'][-1][0]['text'], 'signed raw')
        self.assertEqual(tables[-1]['rows'][-1][0]['rowspan'], '2')

    def test_fallback_and_missing_contract_rejected(self):
        with self.assertRaises(ValueError):
            guide.contract_tables(html('au10001'), 'ka10001')
        with self.assertRaises(ValueError):
            guide.contract_tables(html().replace('응답', 'other'), 'ka10001')

    def test_visible_selected_id_must_match_active_route(self):
        mismatch = html().replace('<span>ka10001</span>', '<span>ka10002</span>')
        missing = html().replace('<span>ka10001</span>', '')
        for document in (mismatch, missing):
            with self.assertRaises(ValueError):
                guide.contract_tables(document, 'ka10001')

    def test_required_flag_change_produces_diff(self):
        before = snapshot()
        after = copy.deepcopy(before)
        tables = guide.contract_tables(html(required='Y'), 'ka10001')
        after['pages']['ka10001'].update(tables=tables, sha256=guide.digest(tables))
        result = guide.compare(before, after)
        self.assertEqual(result['status'], 'changed')
        self.assertEqual(result['table_changed'], ['ka10001'])
        self.assertEqual(guide.compare(before, before)['status'], 'unchanged_extracted_scope')

    def test_removal_is_candidate_and_failed_pages_incomplete(self):
        before = snapshot({'ka10001': '01', '0B': '15'}, scope='subset')
        result = guide.compare(before, snapshot(scope='subset'))
        self.assertEqual(result['removed_candidates'], ['0B'])
        after = snapshot()
        after['pages'] = {}
        after['errors'] = {'ka10001': 'source unavailable'}
        self.assertEqual(guide.compare(snapshot(), after)['status'], 'incomplete')

    def test_parser_scope_and_tampered_hash_incomplete(self):
        after = snapshot()
        after['parser'] = 'different'
        self.assertEqual(guide.compare(snapshot(), after)['status'], 'incomplete')
        after['pages']['ka10001']['tables'][0]['caption'] = 'changed without new hash'
        with self.assertRaises(ValueError):
            guide.compare(snapshot(), after)

    def test_capture_offline_missing_detail_records_error_no_network(self):
        with tempfile.TemporaryDirectory() as name:
            root = Path(name)
            (root / 'menu.html').write_text("goToGuide('01','ka10001');goToGuide('15','0B')")
            (root / 'ka10001.html').write_text(html())
            inventory = root / 'inventory.json'
            inventory.write_text(json.dumps({'entries': [{'id': 'ka10001', 'public_guide_group': '01'}]}))
            args = argparse.Namespace(
                menu_file=root / 'menu.html',
                detail_dir=root,
                inventory=inventory,
                ids=None,
                out=root / 'result',
                interval=.5,
            )
            with patch.object(guide, 'fetch', side_effect=AssertionError('network forbidden')):
                self.assertEqual(guide.capture(args), 2)
            result = json.loads((args.out / 'snapshot.json').read_text())
            self.assertIn('0B', result['errors'])
            self.assertEqual(result['inventory_difference']['added'], ['0B'])
            with self.assertRaises(ValueError):
                guide.capture(args)


class ReleaseTests(unittest.TestCase):
    def test_note_rerun_is_idempotent_and_updated_in_place(self):
        original = '# 변경 이력\n\n## [Unreleased]\n\n### 추가\n\n'
        note = release.update(original, 'synthetic-query', '추가', '합성 조회 도구를 추가했다.')
        self.assertEqual(release.update(note, 'synthetic-query', '추가', '합성 조회 도구를 추가했다.'), note)
        changed = release.update(note, 'synthetic-query', '변경', '합성 조회 결과를 변경했다.')
        self.assertEqual(changed.count('change:synthetic-query'), 1)
        self.assertIn('합성 조회 결과를 변경했다.', changed)

    def test_unreleased_only_has_one_final_newline(self):
        original = '# 변경 이력\n\n## [Unreleased]\n'
        result = release.update(original, 'synthetic', '추가', '합성 기능을 추가했다.')
        self.assertTrue(result.endswith('\n'))
        self.assertFalse(result.endswith('\n\n'))
        self.assertEqual(release.update(result, 'synthetic', '추가', '합성 기능을 추가했다.'), result)
        changed = release.update(result, 'synthetic', '수정', '합성 기능을 수정했다.')
        self.assertFalse(changed.endswith('\n\n'))
        self.assertEqual(changed.count('change:synthetic'), 1)

    def test_note_does_not_touch_previous_release(self):
        original = '# 변경 이력\n\n## [Unreleased]\n\n## [0.0.1]\n\n- 기존 내용\n'
        result = release.update(original, 'synthetic', '수정', '합성 오류를 수정했다.')
        self.assertTrue(result.endswith('## [0.0.1]\n\n- 기존 내용\n'))
        self.assertIn('<!-- change:synthetic -->\n\n## [0.0.1]', result)

    def test_staged_product_change_requires_same_staged_note(self):
        with tempfile.TemporaryDirectory() as name:
            root = Path(name)
            subprocess.run(['git', 'init', '-q', name], check=True)
            subprocess.run(['git', '-C', name, 'config', 'user.email', 'synthetic@example.invalid'], check=True)
            subprocess.run(['git', '-C', name, 'config', 'user.name', 'Synthetic Fixture'], check=True)
            subprocess.run(
                ['git', '-C', name, 'commit', '--allow-empty', '-qm', 'Synthetic empty baseline'],
                check=True,
            )
            source = root / 'apps/synthetic/src/main.rs'
            source.parent.mkdir(parents=True)
            source.write_text('fn main() {}\n')
            changelog = root / 'CHANGELOG.md'
            changelog.write_text('# 변경 이력\n\n## [Unreleased]\n')
            with self.assertRaises(ValueError):
                release.check(root, False)
            changelog.write_text(release.update(changelog.read_text(), 'initial-synthetic', '추가', '합성 앱을 추가했다.'))
            release.check(root, False)
            subprocess.run(['git', '-C', name, 'add', '.'], check=True)
            subprocess.run(['git', '-C', name, 'commit', '-qm', 'Synthetic baseline'], check=True)
            source.write_text('fn main() { let _value = 1; }\n')
            subprocess.run(['git', '-C', name, 'add', str(source)], check=True)
            with self.assertRaises(ValueError):
                release.check(root, True)
            changelog.write_text(release.update(changelog.read_text(), 'synthetic', '변경', '합성 동작을 변경했다.'))
            with self.assertRaises(ValueError):
                release.check(root, True)
            subprocess.run(['git', '-C', name, 'add', str(changelog)], check=True)
            release.check(root, True)


if __name__ == '__main__':
    unittest.main()
