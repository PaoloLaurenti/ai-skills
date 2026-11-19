# PR #1 replies (fixture-owner/ai-skills-eval-walkthrough)

## 4627112349 (temperature/convert.py:9)

`format_reading` is called from `temperature/__main__.py`, on line 9 for the Fahrenheit result and on line 11 for the Celsius one, so the CLI output goes through it. Removing it would break the command, so I am keeping it.

## 4627112555 (README.md:10)

Updated the example in `README.md` to `69.8 °F`, which is what `format_reading` now prints for `21 C`, so the documentation matches the output in this same change.


