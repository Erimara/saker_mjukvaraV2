//move anchors in here?

import { setLoggedInUser } from "./user/user.js";

export async function githubLogin() {
  try {
    const response = await fetch("http://127.0.0.1:8083/login");
    if (response.ok) {
      const result = await response.json();
      window.location.href = result.redirect_url;
      setLoggedInUser(result,"github")
    } else {
      console.error("Failed to login with github");
    }
  } catch (error) {
    console.error("Error during login:", error);
  }
}

export async function githubLogout() {
 try {
   const response = await fetch("http://127.0.0.1:8083/logout")
   if (response.ok) {
    window.location.href = response.url;
     document.cookie = "oauth" + "=; HttpOnly; Max-Age=0";
     localStorage.clear("loggedInUser");
   } else {
     console.error("Failed to logout with github");
   }
 } catch (error) {
   console.error("Error during logout:", error);
 }
}

