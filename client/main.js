import {registerUser, login} from "./user.js"
import {displayPosts, postContent, searchPosts} from "./posts.js";

document.getElementById("sign-in").addEventListener("click", async (e) => {
    e.preventDefault();
    const email = document.getElementById("#login-email").value;
    const password = document.getElementById("#login-pass").value;
    await login(email,password);
})
document.getElementById("sign-up").addEventListener("click", async (e) => {
    e.preventDefault();
    const email = document.querySelector('input[name="email"]').value;
    const password = document.querySelector('input[name="password"]').value;
    await registerUser(email,password);
})

document.getElementById("post-content").addEventListener("click", async (e) =>{
    e.preventDefault();
    const date = getCurrentDate();
    const title = document.getElementById("title").value;
    const content = document.getElementById("content").value;
    console.log(title + content);
    await postContent(title,content,date)
})

document.getElementById("search").addEventListener("keyup", async (e) =>{
    e.preventDefault();
    searchPosts();
})

function getCurrentDate(){
    return new Date().toISOString();
}

await displayPosts();
