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

await displayPosts();
