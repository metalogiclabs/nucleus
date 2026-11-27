# Closed direct unary recursion

Three unchanged Arena Beta exports from commit
`60654ab87757ee8b07e3a292fc546f18e8131a27` (Apache-2.0), plus four
local mutations of `070_nRec` that erase or swap the induction hypothesis,
change its recursive target, or replace the base rule.

The native Lean checker at `cb63311203a44983a2326b83fcb16dbbd5c5130a`
accepts all three originals and rejects all four mutations. Nucleus
qualifies their exact bytes in run 45973517526; UNKNOWN is a decline.
