# Bounded parser repair

Owner: root campaign agent. Original source: 6f8ad5fba4ed6f3fb9f32c9930377a0a48e06931.
Original minimized inputs and receipts are preserved in `local-reproducers-baseline`
and `local-reproducers-route-mask-repair` under the campaign evidence root.

The original parsers accepted df available capacity greater than total, interpreted
an ext4 mount without either access-mode token as writable, and panicked when a
backslash before non-ASCII text crossed a UTF-8 slicing boundary. These are F2/F4
parser reproductions; they do not establish that Linux emits these malformed rows.

The bounded repair rejects available/free greater than total for bytes and inodes,
requires exactly one of ro/rw, and decodes octal pathname escapes as bytes with
checked bounds and strict UTF-8 conversion. Malformed mount data now fails the
source instead of producing usable negative evidence. The existing public
`unescape` helper remains total and returns the original string on malformed input;
the acquisition parser uses the checked decoder, never that fallback.

Formalization consideration: the proposition is fail-closed behavior for these
finite malformed-input classes, not completeness of df or procfs grammar. Boundary
regressions plus the preserved original counterexamples suffice for this bounded
change. No concurrency theorem or real-host qualification follows. Legal non-UTF-8
mount names remain unsupported and yield UNKNOWN; a valid pathname is never
silently transformed using lossy UTF-8 conversion. Exact successor source, regression
results, and reproduced guest behavior belong in the final verification receipt.
