## 0.2.0 (Unreleased)

DEPRECATIONS/BREAKING CHANGES:

 - `SlackError` is now `Error` and `SlackResult` is now `Result`. In addition, `ErrorKind` has been
 removed. All `Error` variants have also been renamed, removing the superfluous `Err` prefix:
  - `ErrSlackResp` -> `Slack`
  - `ErrUtf8` -> `Utf8`
  - `ErrFromHex` -> `FromHex`
  - `ErrHexColor` -> `HexColor`
  - `ErrEncoder` -> `Encoder`
  - `ErrCurl` -> `Curl`
