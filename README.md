This is an automated tool in Rust for Mobile App API Penetration Testing that makes the Android APK to enable User-Wide CA certificates allowing its data to be intercepted and used in Burp Suite.

---

Build the project with:
`cargo build --release`

Run binary:
```bash
cd target/release

# arguments: URL, Start ID, End ID
./apk-user-certs  <APK file>

```