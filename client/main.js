import {registerUser, login, logout, sendLogin,sendRegister } from "./user/user.js"
import { displayAllPosts,} from "./posts/posts.js";
import {postContent, sendPost} from "./posts/postMethods.js";
import { searchPosts } from "./posts/search.js";
import {githubLogin, githubLogout} from "./oauth.js"

document.getElementById("sign-in").addEventListener("click", async (e) => {
    e.preventDefault();
    const email = document.getElementById("#login-email").value;
    const password = document.getElementById("#login-pass").value;
    sendLogin(e);
    await login(email,password);
})
document.getElementById("github-login").addEventListener("click", async (e) => {
  e.preventDefault();
  await githubLogin();
});
document.getElementById("github-logout").addEventListener("click", async (e) => {
  e.preventDefault();
  await githubLogout();
});
document.getElementById("sign-up").addEventListener("click", async (e) => {
    e.preventDefault();
    const email = document.querySelector('input[name="email"]').value;
    const password = document.querySelector('input[name="password"]').value;
    sendRegister(e);
    await registerUser(email,password);
})
document.getElementById("sign-out").addEventListener("click", async (e) => {
    e.preventDefault();
    await logout();
})

document.getElementById("post-content").addEventListener("click", async (e) =>{
    e.preventDefault();
    const date = getCurrentDate();
    const title = document.getElementById("title").value;
    const content = document.getElementById("content").value;
    sendPost(e)
    await postContent(title,content,date)
})


document.getElementById("search").addEventListener("keyup", async (e) =>{
    e.preventDefault();
    searchPosts();
})

function getCurrentDate(){
    return new Date().toISOString();
}

await displayAllPosts();
