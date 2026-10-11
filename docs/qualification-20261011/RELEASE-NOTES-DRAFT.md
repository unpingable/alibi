# Unpublished v0.1 notes — release held

This draft is not a release recommendation. No tag, push, publication or merge was performed.

Candidate change: IPv4 route evidence recognizes a default only when destination and netmask are both zero, and rejects malformed masks. Zero-address narrow networks/host routes no longer masquerade as defaults in the tested parser cases. Existing rejected-route semantics and Receipt V2 are preserved.

Qualification is incomplete. Local evidence shows unresolved command timeout and inadequate-evidence acceptance defects. No Linux guest fault campaign or final holdout has executed. Supported release environments have not been established by this campaign. CLEAR is bounded by a named accusation and actual evidence; it never means host health, DNS/service availability or hardware integrity outside that evidence. Resource-starvation and storage-pathology accusations remain deferred.

Publish only after separately reviewed repairs and reproducible guest qualification satisfy the release gates in QUALIFICATION.md. See RESULTS.md for the exact executed scope.
