# t<i>r</i>eas<i>u</i>re che<i>st</i>

## About

Similar to one-time drop but with stored, encrypted files.
Uses XChaCha20-Poly1305 to encrypt files and Argon2id to verify decryption key.

## Status
Heavily wip and not feature complete 

## TODOs
- [x] Rate limit for unsuccessfully trying to download files
- [x] Store MIME type & file name
- [ ] Frontend
- [ ] Clean up code
- [ ] Tests
- [ ] Documentation
- [ ] Memory usage is pretty high. I believe flushing the files affects this and leads to memory peaks. Fix that.

## License
MIT
