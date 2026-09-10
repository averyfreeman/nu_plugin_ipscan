#!/usr/bin/env nu

# Validate the examples that do not require sending packets. Run this from a
# Nushell session with the `ipscan` plugin registered.
def assert [condition: bool, message: string] {
	if not $condition {
		error make { msg: $message }
	}
}

let rows = (ipscan --list)
let columns = ($rows | columns)
let expected_columns = [name index mac ips is_up is_loopback is_broadcast is_multicast]
let missing_columns = ($expected_columns | where {|column| $column not-in $columns})
assert ($missing_columns | is-empty) $'missing expected columns: ($missing_columns | str join ", ")'

let projected = ($rows | where is_up and not is_loopback | select name index mac ips)
assert (($projected | columns) == [name index mac ips]) 'interface projection did not return the expected columns'

print 'Validated nu_plugin_ipscan examples with Nushell.'
