async function getAllPosts() {
    const response = await fetch("http://127.0.0.1:8080/posts");
    return await response.json();
}

export async function displayPosts() {
    const posts = await getAllPosts();
    const postsList = document.getElementById("postsList");

    posts.forEach(post => {
        const anchor = document.createElement("a");
        anchor.setAttribute("href", "#")
        const h4 = document.createElement("h4");
        const p = document.createElement("p");
        const b = document.createElement("b");
        h4.textContent = post.title;
        p.textContent = post.content;
        b.textContent = post.date;
        anchor.appendChild(h4)
        postsList.appendChild(anchor);
        postsList.appendChild(p);
        postsList.appendChild(b);
    });
}

export async function postContent(title, content, date){
    try {
        const response = await fetch("http://127.0.0.1:8080/create_post", {
            method:"POST",
            headers:{"Content-Type": "application/json"},
            body: JSON.stringify({title,content,date})
        });
        await response.json();
    } catch (error){
        console.log("Error at posting content", error)
    }
}

export function searchPosts(){
    const input = document.getElementById("search").value.toLowerCase();
    const postsList = document.getElementById("postsList");
    const searchList = document.getElementById("search-list");

    searchList.innerHTML = "";

    let titles = postsList.querySelectorAll('h4');

    titles.forEach(title => {
        let searchValue = title.textContent.toLowerCase();

        if (searchValue.includes(input)) {
            const listItem = document.createElement("li");
            listItem.textContent = title.textContent;
            searchList.appendChild(listItem);
            listItem.style.display = "block";
            console.log("found", searchValue);
        }
    });

    if (searchList.children.length > 0 && input !== "") {
        searchList.style.display = "block";
    } else {
        searchList.style.display = "none";
    }
}