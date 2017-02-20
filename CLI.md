```
$ rat {yesterday, today, tomorrow, agenda[--start=<today midnight> --end=<tomorrow midnight>]} [--verbose # shows UUIDs]

All Day
    Payday
    Example (w/10 others)

_fifteen minutes ago: Standup [w/9 others]_
in 26 minutes: Meet with Crew [w/Jim, Kyle]
```

```
$ rat {accounts, accs}

[0] <local> ratrace
[1] <Google> test@gmail.com (Last sync 1 minute ago)
[2] <Google> othertest@somedomain.com (Last sync FAILED 2 days ago)
```

```
$ rat {calendars, cals} <account id or name, ex: test@gmail.com>

[0] Personal (private)
[1] Birthdays (private)
[2] US Holidays (public)
```

```
$ rat {invites,invitations}

# Most recent will always be 0

[0] Standup (Every Wednesday at 10:00) [w/9 others]
```

```
$ rat {accept, decline} <id=0>

Standup [w/9 others] added to your calendar for Every Wednesday at 10:00

>> OR

Standup (Every Wednesday at 10:00) conflicts with Breakfast (Every Weekday at 10:15)

[0] Do Nothing
[1] Decline Standup
[2] Decline Breakfast
[3] Move Standup
[4] Move Breakfast

Which is your preference? [0]:

(2-3 only available if owner of events)
```

```
$ rat import ~/Downloads/event.ics [--auto-accept=<config | yes>]

# This will throw Accept/Decline questions if conflict
# This will add to top of accept/decline stack if not --auto-accept
```

```
$ rat export <event id> <filename=STDOUT>
```

```
$ rat sync <account id=all> <calendar id=all>

Syncing remote account "gmail"...
	- Google Calendar... Done!
	- OOO... Done!
	- FIRST Events... Done!
```

```
$ rat prune <account id=all> [--force]

Calendars stored in database but not found in config:

[0] local: Things I Don't Care About Old
[1] gmail: Something Deleted In Web UI

Remove these from local database only? [y/N/<numbers, comma-separated>]

# This will remove calendars under account (or all accounts, if unspecified)
# which are not defined in daemon.config.toml, from the local data store. This
# currently does not remove anything from remote upstreams, but is still useful
# for remote accounts (ex. calendar was deleted in Google Calendar Web UI but
# is still taking up disk space in ratrace).
```
