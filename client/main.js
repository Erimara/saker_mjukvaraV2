import {
  registerUser,
  login,
  logout,
  sendLoginGoogleData,
  sendRegisterGoogleData,
} from "./user/user.js";
import { displayAllPosts,} from "./posts/posts.js";
import {postContent, sendPost} from "./posts/postMethods.js";
import { searchPosts } from "./posts/search.js";
import {githubLogin, githubLogout} from "./oauth.js"

document.getElementById("sign-in")
.addEventListener("click", async (e) => {
    e.preventDefault();
    const email = document.getElementById("#login-email").value;
    const password = document.getElementById("#login-pass").value;
    sendLoginGoogleData(e);
    await login(email,password);
})
document.getElementById("github-login")
.addEventListener("click", async (e) => {
  e.preventDefault();
  await githubLogin();
});
document.getElementById("github-logout")
.addEventListener("click", async (e) => {
  e.preventDefault();
  await githubLogout();
});
document.getElementById("sign-up")
.addEventListener("click", async (e) => {
    e.preventDefault();
    const email = document.querySelector('input[name="email"]').value;
    const password = document.querySelector('input[name="password"]').value;
    sendRegisterGoogleData(e);
    await registerUser(email,password);
})
document.getElementById("sign-out")
.addEventListener("click", async (e) => {
    e.preventDefault();
    await logout();
})

document.getElementById("post-content")
.addEventListener("click", async (e) =>{
    e.preventDefault();
    const date = getCurrentDate();
    const title = document.getElementById("title").value;
    const content = document.getElementById("content").value;
    sendPost(e)
    await postContent(title,content,date)
})


document.getElementById("search")
.addEventListener("keyup", async (e) =>{
    e.preventDefault();
    searchPosts();
})
document.querySelector(".consent-button")
.addEventListener("click", async (e) => {
  e.preventDefault();
  document.getElementById("popup-wrapper").style.display = "none"
  localStorage.setItem("consent_to_cookies", "true")
});

document.querySelector(".no-consent-button")
  .addEventListener("click", async (e) => {
    e.preventDefault();
    document.getElementById("popup-wrapper").style.display = "none";
    localStorage.setItem("no_consent_to_cookies", "false");
  });


function loadPopup(){
    const consent = localStorage.getItem("consent_to_cookies");
    const noConsent = localStorage.getItem("no_consent_to_cookies");
    if(consent){
     let recaptcha = document.createElement("script");
     let head = document.getElementById("head");
        recaptcha.src = "https://www.google.com/recaptcha/enterprise.js?render=6LeFqVgpAAAAANzbXhYcFL9_9bKs6L9VAY0p6aVy"
        head.appendChild(recaptcha);
        document.getElementById("popup-wrapper").style.display = "none";
    }
    if(noConsent){
        document.getElementById("popup-wrapper").style.display = "none";
    }

}
function getCurrentDate(){
    return new Date().toISOString();
}

await displayAllPosts();
loadPopup();
