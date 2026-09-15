# Security Policy

## Supported versions

Security fixes are applied to the current supported 2.0 release line.

| Version | Support status |
| --- | --- |
| `2.0.x` | Supported |
| `1.0.x` | Unsupported; upgrade to 2.0 where possible |
| Older versions | Unsupported |

## Reporting a vulnerability

Do not open a public issue for a suspected vulnerability. Report it privately to `security@techscript.is-a.dev`.

Please include:

- A clear description of the affected component and potential impact.
- The affected version, platform, and configuration.
- Reproduction steps or a minimal proof of concept.
- Any known mitigation or proposed fix.
- Your preferred attribution name, if you want to be credited.

Remove secrets, credentials, personal information, and unrelated customer data before sending a report. If the report contains sensitive attachments, mention that in the initial message rather than posting them publicly.

## Response process

Maintainers will acknowledge a report when practical, validate the issue, assess its severity, and coordinate a fix or mitigation. The disclosure timeline will depend on exploitability, affected users, availability of a fix, and coordination with reporters or downstream distributors.

We may request additional details, provide a temporary workaround, publish an advisory, or credit the reporter after disclosure. Please do not publicly disclose the issue until maintainers have had a reasonable opportunity to investigate and release a fix.

## Supply-chain and dependency reports

Reports involving third-party dependencies, release artifacts, package publishing, or compromised credentials should also be sent privately. Include the package name, version, artifact digest or URL, and evidence that helps maintainers reproduce or verify the concern.

For non-security bugs, use the [bug report template](.github/ISSUE_TEMPLATE/bug_report.md). For general questions, use [Discussions](https://github.com/Tcode-Motion/techscript/discussions).
