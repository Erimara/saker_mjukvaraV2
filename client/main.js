async function getAllPosts() {
    const response = await fetch("http://127.0.0.1:8080/posts");
    return await response.json();
}

async function displayPosts() {
    const posts = await getAllPosts();
    const postsList = document.getElementById("postsList");

    posts.forEach(post => {
        const h4 = document.createElement("h4");
        const p = document.createElement("p");
        const b = document.createElement("b");
        h4.textContent = post.title;
        p.textContent = post.content;
        b.textContent = post.date;
        postsList.appendChild(h4);
        postsList.appendChild(p);
        postsList.appendChild(b);
    });
}

async function registerUser(email, password) {
    try {
        const response = await fetch("http://127.0.0.1:8080/register", {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({ email, password })
        });

        await response.json();

    } catch (error) {
        console.error("Error during registration:", error);
    }
}
document.getElementById("sign-up").addEventListener("click", async (e) => {
    e.preventDefault();
    const email = document.querySelector('input[name="email"]').value;
    const password = document.querySelector('input[name="password"]').value;

    await registerUser(email,password);
})

await displayPosts();
