# Crate that holds basic types for the various parts of the prediction-ledger modules

What belongs here:
1. Types that represent domain models (predictions) and errors
2. Business logic shared by all modules (e.g. 'validate_certainty')
3. Traits/interfaces that abstract the IO between modules
4. Shared dependencies (e.g. all modules need to use the same version of `serde`)

What doesn't belong here:
1. Implementations of actual IO that aren't shared by all modules
