# release_verifier fixtures

The release verifier tests create candidate roots and first-success reports in
temporary directories. GitHub API responses are monkeypatched in-process so the
tests do not create release evidence and do not depend on network state.
