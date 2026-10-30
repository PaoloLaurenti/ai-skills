# Test doubles

A test double is anything a test puts in place of real code, so the test controls a
dependency instead of reaching the real one. When to use which, and how each is named,
is in the skill body; this file holds what each kind is.

## Kinds

- **Dummy**: does nothing. It only fills a parameter the path under test never uses, so
  it proves nothing about that parameter.
- **Stub**: answers exactly as the test instructed, with canned values, and does nothing
  else. It steers the code down the path under test.
- **Spy**: records the calls it receives, so the Assert block can read the interaction
  history and compare it by equality with the expected calls.
- **Mock**: like a spy, but programmed beforehand with expectations about the calls, and
  verifying them itself.
- **Fake**: a simplified working implementation of the real dependency, such as an
  in-memory repository or a fake payment gateway.
