Yeah, **Alibi** is too good to bury internally.

And **mean time to innocence** is the thing that makes it more than a cute name. That’s an actual operational concept:

> **Alerting tells you why a host might be guilty. Alibi tells you how quickly you can establish that it isn’t.**

That’s a surprisingly under-served problem. Most operational tooling is optimized around positive evidence of failure: alerts, incidents, anomalies, threshold violations. But there’s a very real complementary question:

**“This thing looks suspicious, or simply hasn’t complained lately—can I prove it is presently sane?”**

Which implies a nice bounded object:

* interrogate the host directly
* verify expected services/processes/resources
* check recent error surfaces and latent degradation
* inspect things ordinary alerting may have missed
* distinguish **healthy**, **degraded-but-unalerted**, **unknown**, and **actually busted**
* produce an evidence-bearing receipt rather than merely `PASS`

And MTI is wonderfully perverse because traditional ops metrics are all about failure and restoration:

**MTTD → MTTA → MTTR**

Alibi adds:

**MTTI: Mean Time To Innocence**

“How long does it take us to stop suspecting the machine?”

That is absolutely public-tool material.

The only annoyance is that `alibi` will be a crowded name in software generally, so the *product name* can be Alibi while the repo/package/binary may need something more specific—`alibi-host`, `alibi-ops`, whatever. But I would protect the plain-English name and the MTI concept.

Also, aesthetically, this fits neutral.zone almost suspiciously well: **don’t assume guilt from absence of evidence; interrogate the system and establish what you can actually support.**

Yeah. Kimi is exactly the right victim for this because there are two separable jobs:

**1. Product/research shape**
What exactly counts as “innocence”? What evidence classes matter? What does “unknown” mean? How do you avoid false reassurance? What is MTI measuring?

**2. Implementation shape**
Host interrogator, pluggable checks, bounded evidence collection, result taxonomy, receipts, maybe remote transport later.

I would not let it start by writing code. First prompt should force it to pin down the object and failure model so “Alibi” doesn’t quietly become Yet Another Health Check Script.

The seed is strong enough already:

> **Alibi: establish mean time to innocence for hosts.**
>
> Instead of waiting for alerts to prove a machine is broken, actively interrogate a host to determine whether it is actually healthy, degraded in ways alerting missed, or still epistemically unresolved. Produce evidence-backed findings and a receipt of what was checked.

That’s worth preserving almost verbatim before anyone gets clever with it.

Yeah, that taxonomy is actually good. Annoyingly good.

Those names work because they’re **operator-legible failure classes**, not implementation trivia:

* **Kernel Scream Test** — dmesg, MCE, kernel oopses, watchdogs, PCIe/AER noise
* **Silent Killer** — read-only filesystems, remounts, inode exhaustion, weird mount states, journal issues
* **Network Choke Point** — link flaps, packet loss, DNS weirdness, route asymmetry, conntrack/resource exhaustion
* **Memrot** — EDAC counters, corrected/uncorrected ECC, memory pressure pathologies

That’s exactly the right level. Not “run these 47 commands”; rather: **what class of hidden failure are we trying to exonerate the host from?**

And the important bit is that Alibi should probably report something like:

> `silent-killer: clear`
> evidence: root fs rw, no remount-ro events, inode usage 41%, journal clean

not merely:

> `check_filesystem.sh: PASS`

Because then the tool retains the concept even if the implementation changes.

Gemini may have accidentally found the product grammar here: **named accusations, evidence collection, verdict, receipt**.

That’s a lot stronger than “healthcheck framework.”

