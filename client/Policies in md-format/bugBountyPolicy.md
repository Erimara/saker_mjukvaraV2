# Policy
We take security seriously at anonymous, and we’re committed to protecting our community. If you are a security researcher or expert and believe you’ve identified security-related issues with anonymous’s website or apps, we would appreciate you disclosing it to us responsibly.
I am committed to addressing all security issues in a responsible and timely manner, and we ask the security community to give us the opportunity to do so before disclosing them publicly. Please submit a detailed description of the issue to us, along with the steps to reproduce it.

# When testing
Please include a header Test-anonymous: <anon_(your username)> when you test so we can identify your requests easily.

# Scope
The scope of issues is limited to technical vulnerabilities on the anonymous website. Please do not attempt to compromise the safety or privacy of our users. We also request you not to use vulnerability testing tools that generate a significant volume of traffic.

# Rewards
We will reward reports according to the severity of their impact on a case-by-case basis as determined by our security team. We may pay more for unique, hard-to-find bugs and less for bugs with complex prerequisites that lower the risk of exploitation.
Below, you can find examples of vulnerabilities and their impacts grouped by our severity ranking. This is not an exhaustive list and it is designed to give you insight on how we rate vulnerabilities.

## Critical
* Remote Code Execution (RCE) - able to execute arbitrary commands on a remote device
* Injections - able to read Personally Identifiable Information (PII) or other sensitive data / full read/write access to a database
* Server-Side Request Forgery (SSRF) - able to pivot to internal application and/or access credentials
## High
* Stored Cross-Site Scripting (XSS) - stored XSS with access to cookies and/or other authentication data
* Cross-Site Request Forgery (CSRF) - leading to account takeover
* Account Takeover (ATO) - with no or minimal user interaction
* Insecure Direct Object Reference (IDOR) - read or write access to sensitive data or important fields that you do not have permission to
Injection - able to perform queries with limited access

## Medium
* IDOR - write access to modify objects that you do not have permission to
* XSS - reflected/DOM XSS with access to cookies or other sensitive data

# Eligibility and Responsible Disclosure
To promote the discovery and reporting of vulnerabilities and increase user safety, we ask that you:
Give us a reasonable time to respond to the issue before making any information about it public.
Not access or modify data without the explicit permission of the owner.
Act in good faith not to degrade the performance of our services (including denial of service).
Public disclosure of the vulnerability prior to resolution will result in disqualification from the program. You must report a qualifying vulnerability by filling out a valid report to be eligible for a monetary reward.

# Non-qualifying vulnerabilities / Known Issues
*

# Consequences of complying with this policy
 We will not pursue a civil action or initiate a complaint to law enforcement for accidental, good faith violations of this policy. We consider activities conducted consistent with this policy to constitute “authorized” conduct under the Computer Fraud and Abuse Act (CFAA). We will not bring a DMCA claim against you for circumventing the technological measures we have used to protect the applications in scope.
If legal action is initiated by a third party against you and you have complied with anonymous's bug bounty policy, anonymous will take steps to make it known that your actions were conducted in compliance with this policy.

###  Reports are sent to AnonymousSecurity@anonymous.com 

### Thank you, happy hacking!