# ProxyToggle

[![Current Crates.io Version](https://img.shields.io/crates/v/proxytoggle.svg)](https://crates.io/crates/proxytoggle)
[![Downloads badge](https://img.shields.io/crates/d/proxytoggle.svg)](https://crates.io/crates/proxytoggle)

Systray application to enable or disable proxy. Checks system setting every second to fight eventual group policy.


In organizations, proxy settings can be set by a group policy. But, when you work from home, the proxy must be unchecked. The problem : the policy will always reset it, and that can be really annoying.


Here comes ProxyToggle : you have quick access to set the proxy status (enabled or disabled) in the systray, and the program checks the system proxy setting every second, to set it back to user setting, even if the group policy changed it.
