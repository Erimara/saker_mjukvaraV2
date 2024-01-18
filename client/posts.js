async function getAllPosts() {
    const response = await fetch("http://127.0.0.1:8081/posts");
    return await response.json();
}

export async function displayPosts() {
    const posts = await getAllPosts();
    const postsList = document.getElementById("postsList");

    posts.forEach(post => {
        console.log(post._id);
        const anchor = document.createElement("a");
        anchor.setAttribute("href", "#")
        const h4 = document.createElement("h4");
        const p = document.createElement("p");
        const b = document.createElement("b");
        const button = document.createElement("button");
        button.setAttribute("class", "delete-post");
        h4.textContent = post.title;
        p.textContent = post.content;
        b.textContent = post.date;
        button.textContent = "delete"
        anchor.appendChild(h4)
        postsList.appendChild(anchor);
        postsList.appendChild(p);
        postsList.appendChild(b);
        postsList.appendChild(button);

           button.addEventListener("click", () => {
             deletePost(post._id);
           });

    });
}


export async function postContent(title, content, date){
    try {
        const response = await fetch("http://127.0.0.1:8081/create_post", {
            method:"POST",
            headers:{"Content-Type": "application/json"},
            body: JSON.stringify({title,content,date})
        });
        await response.json();
    } catch (error){
        console.log("Error at posting content", error)
    }
}

 async function deletePost(post_id) {
    const objectId = post_id.$oid;
  try {
    const response = await fetch(
      `http://127.0.0.1:8081/delete_post/${objectId}`,
      {
        method: "DELETE",
        headers: { "Content-Type": "application/json" },
        // No need to include a request body for a DELETE request
      }
    );

    if (response.ok) {
      // Optionally handle success or update UI
      console.log(`Post with ID ${post_id} deleted successfully`);
    } else {
      console.error(`Failed to delete post with ID ${post_id}`);
    }
  } catch (error) {
    console.error("Error at deleting post:", error);
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