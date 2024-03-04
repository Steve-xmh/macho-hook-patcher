# Mach-O Hook Patcher

A utility tool to patch Mach-O binaries to allow code injection by `DYLD_INSERT_LIBRARIES` environment variable.

## Pre-requisites
- a Mac Computer with SIP disabled
- Xcode (for `codesign`)

## How does it work?

The tool just do following steps to remove limitations of `DYLD_INSERT_LIBRARIES`:
- [x] Remove code sign.
- [ ] Rename `__RESTRICT`/`__restrict` to random names.
- [ ] Enable/remove `com.apple.security.cs.allow-dyld-environment-variables` entitlements.
- [x] Resign the program if needed. (Maybe not needed if SIP is disabled)

## References
- [https://theevilbit.github.io/posts/dyld_insert_libraries_dylib_injection_in_macos_osx_deep_dive/](https://theevilbit.github.io/posts/dyld_insert_libraries_dylib_injection_in_macos_osx_deep_dive/)
- [https://developer.apple.com/documentation/bundleresources/entitlements/com_apple_security_cs_allow-dyld-environment-variables?language=objc](https://developer.apple.com/documentation/bundleresources/entitlements/com_apple_security_cs_allow-dyld-environment-variables?language=objc)
- [https://developer.apple.com/documentation/bundleresources/entitlements/com_apple_security_cs_disable-library-validation?language=objc](https://developer.apple.com/documentation/bundleresources/entitlements/com_apple_security_cs_disable-library-validation?language=objc)
