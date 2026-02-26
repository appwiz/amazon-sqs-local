# Code Review

## Scope reviewed
- `src/main.rs`
- `src/appsync/*`
- `tests/appsync_integration.sh`
- `README.md`

## Summary
The new AppSync service wiring and integration test pass end-to-end (`41/41` assertions). I found one correctness issue that can cause partial `untag-resource` behavior.

## Findings

### 1) `untag-resource` only reliably handles a single tag key
**Severity:** Medium  
**File:** `src/appsync/server.rs` (query parsing for `DELETE /v1/tags/{resourceArn}`)

`UntagQuery` models `tagKeys` as `Option<String>` and then splits a single comma-delimited value:

- `tag_keys: Option<String>`
- `s.split(',')...`

AWS list query parameters are commonly sent as repeated keys (for example, `?tagKeys=Project&tagKeys=Env`) instead of one comma-separated value. In that case, only one value may be parsed, so only one tag gets removed.

**Suggested fix:** parse as `Option<Vec<String>>` (or equivalent repeated-query support) and keep comma-splitting only as fallback compatibility if needed.

## Notes
- Verified by running `bash tests/appsync_integration.sh` successfully.
