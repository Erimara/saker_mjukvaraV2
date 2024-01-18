async function getAllPosts() {
    const response = await fetch("http://127.0.0.1:8081/posts");
    return await response.json();
}
async function getPostById(post_id) {
  try {
    const response = await fetch(`http://127.0.0.1:8081/post/${post_id}`);
    if (response.ok) {
      const post = await response.json();
      return post;
    } else {
      console.error(`Failed to get post with ID ${post_id}`);
    }
  } catch (error) {
    console.error("Error at getting post:", error);
  }
}

function displayPostDetails(post) {
  const specificPost = document.getElementById("specific-post");

  const h4 = document.createElement("h4");
  const p = document.createElement("p");
  const b = document.createElement("b");

  h4.textContent = post.title;
  p.textContent = post.content;
  b.textContent = post.date;

  specificPost.appendChild(h4);
  specificPost.appendChild(p);
  specificPost.appendChild(b);
}

document.addEventListener("DOMContentLoaded", async () => {
  const postId = getPostIdFromQueryParams();

  if (postId) {
    const post = await getPostById(postId);
    displayPostDetails(post);
  } else {
    console.error("Post ID not found in query parameters.");
  }
});

function getPostIdFromQueryParams() {
  const urlParams = new URLSearchParams(window.location.search);
  return urlParams.get("post_id");
}

export async function displayPosts() {
  const posts = await getAllPosts();
  const postsList = document.getElementById("postsList");

  posts.forEach((post) => {
    const postContainer = document.createElement("div");
    postContainer.setAttribute("class", "post-container");

    const h4 = document.createElement("h4");
    const p = document.createElement("p");
    const b = document.createElement("b");
    const button = document.createElement("button");
    button.setAttribute("class", "delete-post");

    h4.textContent = post.title;
    p.textContent = post.content;
    b.textContent = post.date;
    button.textContent = "delete";

    postContainer.appendChild(h4);
    postContainer.appendChild(p);
    postContainer.appendChild(b);
    postContainer.appendChild(button);

    postContainer.addEventListener("click", () => {
      redirectToPostPage(post._id.$oid);
    });

    button.addEventListener("click", () => {
      deletePost(post._id);
    });

    postsList.appendChild(postContainer);
  });
}
function redirectToPostPage(postId) {
  // Redirect to post.html with the specific post_id in the query parameter
  window.location.href = `post.html?post_id=${postId}`;
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
      }
    );

    if (response.ok) {
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