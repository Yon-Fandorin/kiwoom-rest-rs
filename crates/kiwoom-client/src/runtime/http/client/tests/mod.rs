use super::*;

// 초 단위 값과 HTTP 날짜를 해석하고 과거 날짜는 즉시 대기로 처리합니다.
#[test]
fn parses_http_date_and_numeric_retry_after() {
    let now = chrono::DateTime::parse_from_rfc2822("Wed, 21 Oct 2015 07:27:00 GMT")
        .unwrap()
        .with_timezone(&chrono::Utc);
    assert_eq!(
        retry_after_seconds("Wed, 21 Oct 2015 07:28:00 GMT", now),
        Some(60)
    );
    assert_eq!(
        retry_after_seconds("Wed, 21 Oct 2015 07:26:00 GMT", now),
        Some(0)
    );
    assert_eq!(retry_after_seconds("120", now), Some(120));
    assert_eq!(retry_after_seconds("unknown", now), None);
}

// HTTP 날짜의 세 가지 표기와 소수초를 같은 서버 대기 시각으로 처리한다.
#[test]
fn all_http_date_forms_and_fractional_waits_are_supported() {
    let now = chrono::DateTime::parse_from_rfc3339("1994-11-06T08:49:00Z")
        .unwrap()
        .with_timezone(&chrono::Utc);
    for date in [
        "Sun, 06 Nov 1994 08:49:37 GMT",
        "Sunday, 06-Nov-94 08:49:37 GMT",
        "Sun Nov  6 08:49:37 1994",
    ] {
        assert_eq!(retry_after_seconds(date, now), Some(37));
        assert_eq!(
            retry_after_seconds(date, now + chrono::Duration::seconds(38)),
            Some(0)
        );
    }
    let before = now + chrono::Duration::seconds(37) - chrono::Duration::nanoseconds(1);
    assert_eq!(
        retry_after_seconds("Sun, 06 Nov 1994 08:49:37 GMT", before),
        Some(1)
    );
    assert_eq!(
        retry_after_seconds("Sun, 99 Nov 1994 08:49:37 GMT", now),
        None
    );
}
