# PrettyBadPrivacy
XOR-encrypt and decrypt from stdin.

> The name is a joke, referring to PGP (Pretty Good Privacy) encryption.
>
> Needless to say, XOR-encryption is incredibly weak and shouldn't be used for sensitive data. Furthermore, the XOR mask used is very simple : `pbpPBP` (cycling).
> 
> This was just a small personnal project to learn Rust.

### Usage
```
# Encryption
cat my_secret_file.dat | pbp > encrypted.pbp

# Decryption
cat encrypted.pbp | pbp -d > my_secret_file.dat
```
