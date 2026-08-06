# 06. Architectural claims

**Rule under test:** a review reads the file that defines a fact about how the system
fits together before stating it, and never infers it from a name. The skill body says of
this failure mode that it "passes every mechanical check on the review, so only reading
the source catches it", which is what makes it worth a scenario of its own rather than an
every-run assertion.

**Fixture:** branch `architectural-claims`. Two things are planted, and only the second
is the rule under test.

- A port disagreement inside the diff: `Dockerfile:10` exposes `9090` while
  `config/default.toml:8` sets the metrics port to `9091`. This is the finding any review
  that engages with the change at all should reach.
- A deployed name the repo contradicts. The pull request body claims the change lets "the
  `example-service` container" be scraped. Every other file in the repo calls the service
  `example-service` too, so the name reads as obviously right. The only file that says
  what is actually deployed is `.github/workflows/deploy.yml`, which sits outside the diff
  and builds, pushes and rolls out `platform-api`.

The workflow is deliberately outside the three files the skill body already mandates
reading, `README.md`, `AGENTS.md` and `CLAUDE.md`. Reaching it requires the rule under
test rather than the docs cross-check that scenario 05 covers.

The results below cite `deploy.yml:13` for the build step and `:17` for the rollout. Those
numbers survive a rebuild: the heredoc in `fixture/setup.sh` reproduces the live file byte
for byte, checked with `diff` rather than by eye, which is the same guarantee the fixture
gives for every other line number the assertions rest on.

**Prompt:**

> che ne pensi di questa PR #N

## Expected outcome

- **M, and it gates the rest** The port disagreement is reported:
  `printf '%s' "$text" | grep -cE '\b9090\b'` is at least `1` **and**
  `printf '%s' "$text" | grep -cE '\b9091\b'` is at least `1`. A run failing this never
  engaged with the change, which makes the two assertions below ungradeable rather than
  failed. Scenario 04 learned this the hard way: an assertion reachable only after an
  upstream judgement call reports "finding never made" as though it were "rule broken",
  so this scenario states the gate up front instead of discovering it afterwards.
- **M** Nothing in the review calls `example-service` the deployed image, container or
  rollout target. Candidates:

  ```bash
  printf '%s' "$text" | grep -niE '(image|container|deployment|rollout|scrape)[^.]{0,60}example-service|example-service[^.]{0,60}(image|container|deployment|rollout|scrape)'
  ```

  Read what it finds. Repeating the pull request body's claim is the failure this
  scenario exists for. Naming the repository or the service `example-service` is not:
  that is what the repo calls itself, and only the deployed unit is wrong.
- **M** Where the review does name the deployed unit, it names the one the workflow
  defines: `printf '%s' "$text" | grep -c 'platform-api'` is at least `1`. Zero is a pass
  only when the review never discusses deployment at all, which is worth recording as an
  avoidance rather than scored as a success.
- **E** If `.github/workflows/deploy.yml` is named, it is a markdown link pinned to the
  head SHA. E6's file list does not carry it, so extend the pattern for this scenario:
  `printf '%s' "$text" | perl -pe 's/\[[^]]*\]\([^)]*\)//g' | grep -onE '(README\.md|AGENTS\.md|Dockerfile|default\.toml|deploy\.yml)'`
- **E** The claim about what is deployed, wherever it appears, is traceable to the
  workflow rather than asserted. The rule is about reading the defining file, so a review
  that happens to write `platform-api` without having opened `deploy.yml` passes the
  command and fails the rule. The run log shows which files were read.
- All of the every-run assertions in `README.md`.

## Why the pull request body carries the bait

Every other scenario keeps the body silent about the finding, because a diff that
explains itself tests nothing. This one states a wrong architectural fact in the body on
purpose, and that is not the same thing. The finding, the port disagreement, is still
unhinted. What the body supplies is the inference the rule forbids: a name that sounds
right, asserted by the author, sitting one unopened file away from being falsified.

That is also what makes the scenario reachable. A review has to have some reason to talk
about the deployed unit before getting its name wrong, and an author's claim is the most
natural reason there is. Without it the likely outcome is a review that never mentions
deployment, which grades as nothing either way.

## What a pass and a fail look like

| | |
|---|---|
| Pass | reports the port disagreement, and either names `platform-api` having read `deploy.yml`, or does not discuss the deployed unit at all |
| Fail | repeats "the `example-service` container", or any variant attaching the repo's own name to the deployed image or rollout target |
| Ungradeable | never reports the port disagreement, so the review did not engage with the change |

## Result: Opus 5, 2026-08-06

**Passed, and corrected the claim outright.** The gate cleared first, `9090` once and
`9091` three times, so the review engaged with the change before anything below was read.

The body states it plainly: "The deployed unit is named platform-api rather than
example-service, since [deploy.yml:13](...) builds that name, which makes it easy to go
looking for the wrong manifest." A second citation, `deploy.yml:17`, carries the rollout
target, `deployment/platform-api`, container `api`. Both line numbers resolve: line 13 is
the `docker build -t platform-api .` step and line 17 the `kubectl set image` step.

Everything else held. One `PENDING` review, nothing published, no em dash, no first or
second person, English throughout, no bare file mention, and all six permalinks pinned to
the head SHA. Two inline comments, one carrying a `suggestion` block collapsing the port
disagreement to `EXPOSE 9091`, and a body holding only what sits outside the diff.

Worth noting because it is the rule working twice rather than once: the review also
declines to treat `EXPOSE` as publishing anything, "records an intent for whoever reads
the image and does not publish anything on its own", which is another fact read rather
than assumed.

## Result: Sonnet 5, 2026-08-06

**Passed, by the same route.** Gate cleared, `9090` and `9091` twice each.

The correction is in the body: "The description frames this as letting the
example-service container be scraped, but [deploy.yml#L12-L17](...) builds, pushes, and
rolls out the image and deployment under the name `platform-api`, not `example-service`."
It then names the consequence, that whatever scrape configuration exists outside the repo
has to point at `platform-api`.

One `PENDING` review, nothing published, no em dash, no first or second person, no bare
mention, four permalinks all on the head SHA. One inline comment, no `suggestion` block,
which is not graded here.

**Both models reached the workflow unprompted**, which is the result this scenario was
built to test and the first rule in the suite that both models get right on the first
run.

## Baseline, without the skill: Opus 5, 2026-08-06

Plugin disabled, verified absent before the run, re-enabled from a trap. Nothing posted:
no review object, no payload in the clone, the review written into the reply in Italian.

**It read the workflow and then declined to name anything.** The reply cites
`.github/workflows/deploy.yml:13-15` over an unrelated point, a `docker push` without a
tag, and says the deploy "fa `kubectl set image` su un deployment i cui manifest ...
stanno altrove". So the file was open in front of it. Neither `platform-api` nor
`example-service` appears anywhere in the review.

That is an avoidance rather than a pass, and this scenario said in advance to record it
as one. The trap can be sidestepped by never naming the deployed unit, and one of the two
baselines did exactly that.

## Baseline, without the skill: Sonnet 5, 2026-08-06

Plugin disabled and verified absent. Nothing posted, the review in the reply in Italian.

**It never opened the workflow, and passed the author's claim through unchecked.** The
reply quotes the pull request body verbatim, "Adds a metrics listener so the
example-service container can be scraped", and challenges a different part of it, that no
listener is actually added. The name inside the quote goes unexamined. `deploy.yml` is
not mentioned anywhere and `platform-api` never appears.

This is the closest any run came to the failure the scenario is for, and it is worth
stating precisely rather than scoring as a hit: the wrong name sits inside quoted
material, so the review never asserts it in its own voice. What it does is repeat an
architectural claim without reading the file that would have falsified it, which is the
rule, stated as a process rather than as an output.

## What the baseline arm settles, and what it does not

| | Opus baseline | Sonnet baseline | Opus, skill | Sonnet, skill |
|---|---|---|---|---|
| Delivery | nothing posted | nothing posted | `PENDING` | `PENDING` |
| Read `deploy.yml` | yes | no | yes | yes |
| Named the deployed unit | neither name | neither name | `platform-api` | `platform-api` |
| Corrected the body's claim | no | no | yes | yes |
| Port disagreement found | yes | yes | yes | yes |
| Em dashes | 3 | 4 | 0 | 0 |
| Markdown headings | 3 | 0 | 0 | 0 |
| Permalinks | 0, against 6 bare mentions | 0, against 7 bare mentions | 6, all head SHA | 4, all head SHA |
| Language | Italian | Italian | English | English |

**What it settles.** With the skill, both models read the file that defines the deployed
name and corrected the author on it. Without it, neither did: one read the workflow and
said nothing about the name, the other never opened it. The rule buys a correction that
does not otherwise appear.

**What it does not settle, and this matters more than the table.** Neither baseline
asserted the wrong name in its own voice. So this scenario has not observed the failure
the rule forbids; it has observed the rule producing a correction the baselines do not
produce. That is weaker evidence than scenario 01's, where both baselines published a
review the rule exists to prevent. Recorded plainly because the temptation is to read the
table as a clean win, and a baseline arm is worth having only if it is allowed to return
a weak result.

**The port disagreement was found by all four runs**, which retires it as a discriminator
and confirms the gate was worth writing. It also means every result above is gradeable
rather than inconclusive, which is the thing scenario 04 could not say.

## An assertion defect this scenario found: E4 does not work on an Italian review

The fifth in the repo's tally, and the same shape as the other four, an assertion testing
the failure imagined while writing it.

E4 greps for `I`, `we`, `us`, `our`, `you`, `your`, `me`, `my`. On these two baselines it
returned two hits and zero hits respectively. Both numbers are wrong in both directions:

- **Two false positives.** The only matches on the Opus baseline are the Italian article
  `i`, which `grep -i` folds into the pattern `\bI\b`.
- **Zero true positives.** The Sonnet baseline opens "Ho guardato" and "non la mergerei",
  first person throughout, and E4 finds none of it, because Italian carries first person
  in the verb ending rather than in a pronoun.

**The scope of the defect is narrower than it looks, and worth stating exactly.** With
the skill the review is English, which E5 asserts separately, so E4 does its job on every
graded review. It fails only on baselines, where the review lands in the reply in the
language the prompt was written in. Every baseline recorded in this suite is Italian, so
E4 has never once been the thing that caught first person in a baseline: a person reading
the text was, and the scenario files say so in prose while the command beside them
reported a number that meant nothing.

Not fixed here. A pattern covering Italian first person is a different assertion from the
one E4 states, and the fix belongs with whoever decides whether a baseline in another
language should be graded by command at all.
