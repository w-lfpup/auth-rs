# Auth-rs

Authentication API for self hosted projects.

What if invitation keeps a potential USER id?

That way when:

an invitation response comes in
there's a contact type and contact info
there's a pregenerated person id

then a user provides screenname and email and password
along with the invitation

so we get a first imprint?
invitation_id
	- contact type
	- contact info
	- person id (could just be the invitation_id?)
- screenname
- display name
- password

Now instead of waiting for a person_id and associating it all
theres an id to assign every part.

set contact with id
set user atomics with id


what about registered or verified attribtues?