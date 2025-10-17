
# System


## Profile
- it can be exported
- requires master passphrase to access

it is a sql-lite file which contains Vaults
, A Profile can be stored locally or in cloud

each profile has unique id if seperate files exists with same unique id it would considered a single Profile and all vaults would be merged in App

it would mean some vaults from same profile can be stored in different cloud services or locally also Vaults can have password , its Key is derived from Master Key and password (if any)

## Vaults

- Can have password
- required Profile Master Key to access
- can  have infinite App Connections for Passwords
- description (optional)
- name (required) indexed for search

it can be moved around in different Files of same parent Profile id , a vault can be a opened in any arbitary Profile with correct Master Key and Password (required),

## App Connections
this is where you store your passwords and other details
- Provider Name (required) indexed for search
- username (optional)
- description (optional)
- email/phone (optional)
- password (required)

TOPT Support
 for 2 factor authetication a time bound OTP can be generated if enabled

the app connections are encrypted with vault Keys which are generated from Master Key and Vault Password
