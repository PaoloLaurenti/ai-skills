# Password reset

People who forget their password can set a new one from the app, without contacting
support.

- On the sign-in page, a "Forgot password?" link opens a page asking for the email
  address of the account.
- After the address is sent, the page always says "If an account exists for this
  address, we sent you a link", whether or not the account exists, so nobody can
  find out which addresses have an account.
- The email holds a link that opens the app on a "New password" page.
- A new password has at least 12 characters and cannot be the current one. The page
  says which rule a refused password breaks.
- Once the new password is saved, the person is signed in on this device, and every
  other device signed in to the account is signed out.
- An address can ask for at most 3 links an hour. A fourth request shows "Too many
  requests, try again later" and sends nothing.
- How long a link stays valid has not been decided yet.

Design: the "Password reset" page of the app's design file.
