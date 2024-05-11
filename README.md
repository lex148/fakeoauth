
# Fake OAuth

Fake OAuth is an OAuth2 provider that you can use while developing. 
It is a zero config solution that provides zero security.

FakeOAuth implements the OAuth protocol so you can develop your site without having to setup a real OAuth provider.

When your ready to go to production, your just a config change away from real OAuth

## Using FakeOAuth

You can run FakeOAuth from docker like this:
```
docker run --rm -p "127.0.0.1:5860:5860" lex148/fakeoauth
```

Send your login requests to `http://127.0.0.1:5860/` and your good to go 
