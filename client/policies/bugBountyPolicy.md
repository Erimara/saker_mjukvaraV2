# Policy
We take security seriously at Zomato, and we’re committed to protecting our community. If you are a security researcher or expert and believe you’ve identified security-related issues with Zomato’s website or apps, we would appreciate you disclosing it to us responsibly.
Our team is committed to addressing all security issues in a responsible and timely manner, and we ask the security community to give us the opportunity to do so before disclosing them publicly. Please submit a detailed description of the issue to us, along with the steps to reproduce it. We trust the security community to make every effort to protect our users’ data and privacy.

# Disclosure Policy
Let us know as soon as possible upon discovery of a potential security issue, and we'll make every effort to quickly resolve the issue.
Provide us a reasonable amount of time to resolve the issue before any disclosure to the public or a third-party.
Make a good faith effort to avoid privacy violations, destruction of data, and interruption or degradation of our service. Only interact with accounts you own or with the explicit permission of the account holder.

# Test Plan
Please include a header X-Hackerone: <h1_username> when you test so we can identify your requests easily.
Scope
The scope of issues is limited to technical vulnerabilities in the Zomato website or mobile apps. Please do not attempt to compromise the safety or privacy of our users (so please use test accounts), or the availability of Zomato through DoS attacks or spam. We also request you not to use vulnerability testing tools that generate a significant volume of traffic.
Certain vulnerabilities with a working proof of concept on some of our Android mobile app(s) may qualify for an additional bounty through the [Google Play Security Rewards Program] (https://hackerone.com/googleplay). To see which apps and vulnerabilities may qualify for a bounty, please refer to the [Google Play Security Rewards Program’s Scope and Vulnerability Criteria] (https://hackerone.com/googleplay).

# Rewards
We will reward reports according to the severity of their impact on a case-by-case basis as determined by our security team. We may pay more for unique, hard-to-find bugs; we may also pay less for bugs with complex prerequisites that lower the risk of exploitation.
Below, you can find examples of vulnerabilities and their impacts grouped by our severity ranking. This is not an exhaustive list and it is designed to give you insight on how we rate vulnerabilities.

## Critical
Remote Code Execution (RCE) - able to execute arbitrary commands on a remote device
SQL Injection - able to read Personally Identifiable Information (PII) or other sensitive data / full read/write access to a database
Server-Side Request Forgery (SSRF) - able to pivot to internal application and/or access credentials (not blind)
Information Disclosure - mass PII leaks including data such as names, emails, phone numbers and addresses
## High
Stored Cross-Site Scripting (XSS) - stored XSS with access to non HttpOnly cookies
Information Disclosure - leaked credentials
Subdomain Takeover - on a domain that sees heavy traffic or would be a convincing candidate for a phishing attack
Cross-Site Request Forgery (CSRF) - leading to account takeover
Account Takeover (ATO) - with no or minimal user interaction
Insecure Direct Object Reference (IDOR) - read or write access to sensitive data or important fields that you do not have permission to
SQL Injection - able to perform queries with a limited access user
## Medium
CSRF - able to modify important information (authenticated)
ATO - required user interaction
IDOR - write access to modify objects that you do not have permission to
XSS - reflected/DOM XSS with access to cookies

## Low
Directory listings
XSS - POST based XSS (with CSRF bypass)
Lack of HTTPS on dynamic pages (judged on a case-by-case basis)
Server information page (no credentials)
Subdomain Takeover - on an unused subdomain

# Eligibility and Responsible Disclosure
To promote the discovery and reporting of vulnerabilities and increase user safety, we ask that you:
Give us a reasonable time to respond to the issue before making any information about it public.
Not access or modify data without the explicit permission of the owner.
Act in good faith not to degrade the performance of our services (including denial of service).
We only reward the first reporter of a vulnerability. Public disclosure of the vulnerability prior to resolution will result in disqualification from the program. You must report a qualifying vulnerability through the HackerOne reporting tool to be eligible for a monetary reward.
# Non-qualifying vulnerabilities / Known Issues
When reporting vulnerabilities, please consider (1) attack scenario/exploitability, and (2) the security impact of the bug. The following issues are considered out of scope:
Google Maps API Keys Leakage
Clickjacking on pages with no sensitive actions
Cross-Site Request Forgery (CSRF) on unauthenticated forms or forms with no sensitive actions
Attacks requiring MITM or physical access to a user's device.
Previously known vulnerable libraries without a working Proof of Concept.
Comma Separated Values (CSV) injection without demonstrating a vulnerability.
Missing best practices in SSL/TLS configuration.
Any activity that could lead to the disruption of our service (DoS).
Content spoofing and text injection issues without showing an attack vector/without being able to modify HTML/CSS
Rate limiting or brute force issues
Invalidation/expiry on CDN assets
Missing best practices in Content Security Policy.
Missing HttpOnly or Secure flags on cookies
Missing email best practices (Invalid, incomplete or missing SPF/DKIM/DMARC records, etc.)
Vulnerabilities only affecting users of outdated or unpatched browsers [Less than 2 stable versions behind the latest released stable version]
Software version disclosure / Banner identification issues / Descriptive error messages or headers (e.g. stack traces, application or server errors).
Public Zero-day vulnerabilities that have had an official patch for less than 1 month will be awarded on a case by case basis.
Tabnabbing
Open redirect - unless an additional security implication can be demonstrated
Self XSS
Promo code abuse (e.g. ordering multiple times using the same promo code)
Abuse of our promotional offers and referral codes
CSRF on www.zomato.com/php/ and www.zomato.com/clients/
Promo code enumeration, abuse of our promotional offers and referral codes.
Able to retrieve user's public information.
Username / email enumeration

# Consequences of complying with this policy
 We will not pursue a civil action or initiate a complaint to law enforcement for accidental, good faith violations of this policy. We consider activities conducted consistent with this policy to constitute “authorized” conduct under the Computer Fraud and Abuse Act (CFAA). We will not bring a DMCA claim against you for circumventing the technological measures we have used to protect the applications in scope.
If legal action is initiated by a third party against you and you have complied with Zomato's bug bounty policy, Zomato will take steps to make it known that your actions were conducted in compliance with this policy.

### Please submit a HackerOne report to us before engaging in conduct that may be inconsistent with or unaddressed by this policy.
### Thank you for helping keep @Zomato safe for the community!