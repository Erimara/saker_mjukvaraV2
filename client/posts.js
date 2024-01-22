import {
  deletePost,
  getAllPosts,
  getPostById,
} from "./postMethods.js";


function displayPostById(post) {
  const specificPost = document.getElementById("specific-post");

  const h4 = document.createElement("h4");
  const p = document.createElement("p");
  const i = document.createElement("i");
  const button = document.createElement("button");
  button.addEventListener("click", () => {
    deletePost(post._id);
  });
  appendText(button, h4, p, i, specificPost, post);
}

export async function displayAllPosts() {
  const posts = await getAllPosts();
  const postsList = document.getElementById("postsList");

  posts.forEach((post) => {
    const postContainer = document.createElement("div");
    postContainer.setAttribute("class", "post-container");
    const h4 = document.createElement("h4");
    const p = document.createElement("p");
    const i = document.createElement("i");
    const button = document.createElement("button");
    button.setAttribute("class", "delete-post");
    appendText(button, h4, p, i, postContainer, post);
    
    h4.addEventListener("click", () => {
      redirectToPostPage(post._id.$oid);
    });

    button.addEventListener("click", () => {
      deletePost(post._id);
    });

    postsList.appendChild(postContainer);
  });
}

function appendText(button, h4, p, i, postContainer, post) {
  button.setAttribute("class", "delete-post");
  h4.innerText = post.title;
  p.innerText = post.content;
  i.innerText = `Posted at: ${post.date}`;
  button.innerText = "delete";

  postContainer.appendChild(h4);
  postContainer.appendChild(p);
  postContainer.appendChild(i);
  postContainer.appendChild(button);
}

document.addEventListener("DOMContentLoaded", async () => {
  const postId = getPostIdFromQueryParams();

  if (postId) {
    const post = await getPostById(postId);
    displayPostById(post);
  }
});

function getPostIdFromQueryParams() {
  const urlParams = new URLSearchParams(window.location.search);
  return urlParams.get("post_id");
}
function redirectToPostPage(postId) {
  window.location.href = `post.html?post_id=${postId}`;
}