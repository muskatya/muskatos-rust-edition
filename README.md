# MuskatOS Rust Edition
MuskatOS Rust Edition is a port of MuskatOS to Rust. At the moment, MOSRE doesn't support Memu, and it has no file system at all. In the future, it will be a full port of MOS to Rust, but for now, it is a very stripped-down version of MuskatOS. When you are installing MuskatOS Rust Edition you are getting:
- Real executable file of stripped-down version of MuskatOS
- Future Memu support and all functions of the original MuskatOS
# Installation guide
Download the executable, run it, and log in as the default root user (default password: root).
# What's new:
- Made the shell look like the one in the latest original versions
- Added a user system. Users are no longer created at startup — the root user is now pre-created (just like in original versions 1.0–1.1.2 I believe). User data isn't saved anywhere yet; planning to store it in JSON in the future
