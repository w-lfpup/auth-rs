# Auth-rs


ACTIONS

- rate limit ip

- create guest session
    - rate limit session creation from ip address
    - create session
    - (also rate limit session)

- signup

- create account
    - create person
    - create password
    - create totp?
    - create session

- login
    - requires a guest session?
    - rate limit login attempts
    - create user session

- verify session
    - check if exists
    - does the public session match the db
    - (on write? is this session still active)
    - rate limit session

- logout
    - delete session
    - attempt to delete all public sessions


- reset password
- add contact
