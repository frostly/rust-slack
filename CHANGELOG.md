## 0.2.0 (Unreleased)

DEPRECATIONS/BREAKING CHANGES:

 - `Payload`
  - `PayloadTemplate` is removed.
  - `unfurl_links` is now bool. No integer transformation is required by the slack API anymore.
 - `Attachment`
  - `AttachmentTemplate` is removed.
  - `color` is now optional.
 - `SlackError` is now `Error` and `SlackResult` is now `Result`. In addition, `ErrorKind` has been
 removed. All `Error` variants have also been renamed, removing the superfluous `Err` prefix:
  - `ErrSlackResp` -> `Slack`
  - `ErrUtf8` -> `Utf8`
  - `ErrFromHex` -> `FromHex`
  - `ErrHexColor` -> `HexColor`
  - `ErrEncoder` -> `Encoder`
  - `ErrCurl` -> `Curl`
 - `SlackLink`, `SlackText`
  - The `Display` trait is now used to format strings for sending to slack rather than `Debug`.
  The `Debug` impl is derived now.
 - `Slack::new` now returns a `Result<Slack>` as it does `Url` parsing.

FEATURES

 - `PayloadBuilder` and `AttachmentBuilder` should be used for building a `Payload` or `Attachment`
 respectively. Errors won't be returned until the final `build` function is called. At this point,
 only the first error is displayed. Subsequent errors will only appear once the first error is
 fixed.
 - `Payload`
  - Added `unfurl_media` option
 - Three character hex color codes, e.g. `#000`, are now supported.
 - Add support for sending links and text into slack, see:
 [Text with Links](README.md#text-with-links).
 - Optional fields no longer are sent in the serialized json message to slack.
 - Add `parse` option to `Payload`.
 - New `Attachment` fields have been added.

OTHER

 - `TryFrom` and `TryInto` traits have been added temporarily to this crate until they are
 formalized in rust proper. See https://github.com/rust-lang/rust/issues/33417 for details.
 - `HexColorT` trait removed. Conversions are used instead.
