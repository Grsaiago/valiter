# Valiter
A validation library inspired by method chaining and typescript's [Zod](https://zod.dev/)

**This is an experimental library, there is no current major version and the api may change drastically between minor releases.**

## Roadmap
### Common Zod validations
The first goal is to mirror all validations from zod for all fundamental types (String/str, i<n> u<n> f<n>).

### Strings

Basic validations:

- [x] min
- [x] max
- [x] length
- [x] startsWith
- [x] endsWith
- [x] uppercase
- [x] lowercase
- [x] with custom function
- [ ] includes
- [ ] regex (feature gated?)

Format validations:
- [x] email
- [ ] uuid
- [ ] url
- [ ] httpUrl       // http or https URLs only
- [ ] hostname
- [ ] emoji         // validates a single emoji character
- [ ] base64
- [ ] base64url
- [ ] hex
- [ ] jwt
- [ ] nanoid
- [ ] cuid
- [ ] cuid2
- [ ] ulid
- [ ] ipv4
- [ ] ipv6
- [ ] mac
- [ ] cidrv4        // ipv4 CIDR block
- [ ] cidrv6        // ipv6 CIDR block
- [ ] hash("sha256")  // or "sha1", "sha384", "sha512", "md5"
- [ ] iso_date
- [ ] iso_time
- [ ] iso_datetime
- [ ] iso_duration
