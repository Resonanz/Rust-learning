Seems I also need udev rules to provide permissions.

Thats just how this works on linux.

If your device is not yet handled by one of the many already shipped rules in your distro, then you have to add your own rule.

If you intend to write an app for a particular device, write a rule with its VID / PID and install it as part of the your application.
