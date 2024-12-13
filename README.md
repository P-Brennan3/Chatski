# Chatski

Chatski is an open source encrypted terminal based chat application written in Rust. It allows developers
to chat with another way in an ephemeral and secure manner. 

* Messages can only be decrypted once
* Messages not yet decrypted (or previously viewed) are shown as scrambled random content
* Messages are never peristed
* Inactive teammembers can't view chat history
* Alerts when teammembers are 'viewing' (decrypting) a message for an extended period of time (indicates suspicious behavior)

* not yet viewed- ```[▀_▀]```
* previously viewed- ```@#$%&*!?/\|[]{}=+-_<>~^```

## TODO
* use git usernames instead
* keep track of users in the chat
* format messages better (username, timestamp)
* encrypt/decrypt logic (start with just encrypt and permanent decrypt)
* keep track of what users decrypted which messages
* update logic to be temporary decryption
* alerts for suspicious behavior

