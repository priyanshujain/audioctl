# audioctl

Virtual audio devices and an echo-cancelling bridge for macOS. `audioctl` installs two Core Audio HAL plug-ins
(Workstation Speaker and Workstation Mic) and runs a bridge that cancels the speaker's echo from the real microphone
before other apps hear it. `audioctl-ui` is a menu bar app for the same bridge.

```bash
nix profile install github:priyanshujain/audioctl
```

GPL-3.0-or-later. See [LICENSE](LICENSE).
